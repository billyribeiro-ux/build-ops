use crate::db::models::import::{AiAnalysisResponse, ChunkedDocument};
use crate::error::AppError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
const MODEL: &str = "claude-sonnet-4-20250514";
const MAX_TOKENS: usize = 8192;
const TEMPERATURE: f32 = 0.3;
const MAX_RETRIES: u32 = 3;

#[derive(Debug, Clone)]
pub struct AiConfig {
    pub api_key: String,
}

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: usize,
    temperature: f32,
    system: String,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    text: String,
}

pub async fn analyze_with_ai(
    config: &AiConfig,
    chunked: &ChunkedDocument,
) -> Result<AiAnalysisResponse, AppError> {
    let client = Client::builder()
        .timeout(Duration::from_secs(300))
        .build()
        .map_err(|e| AppError::ExternalServiceError(format!("HTTP client error: {}", e)))?;

    if chunked.chunks.len() == 1 {
        return analyze_single_chunk(config, &client, &chunked.chunks[0].content).await;
    }

    let mut chunk_results = Vec::new();
    for chunk in &chunked.chunks {
        let result = analyze_single_chunk(config, &client, &chunk.content).await?;
        chunk_results.push(result);
    }

    merge_chunk_results(config, &client, chunk_results).await
}

async fn analyze_single_chunk(
    config: &AiConfig,
    client: &Client,
    content: &str,
) -> Result<AiAnalysisResponse, AppError> {
    let system_prompt = get_system_prompt();
    let user_message = format!("Analyze this curriculum content and generate a structured learning plan:\n\n{}", content);

    let request = AnthropicRequest {
        model: MODEL.to_string(),
        max_tokens: MAX_TOKENS,
        temperature: TEMPERATURE,
        system: system_prompt,
        messages: vec![Message {
            role: "user".to_string(),
            content: user_message,
        }],
    };

    let mut last_error = None;
    for attempt in 1..=MAX_RETRIES {
        match send_request(config, client, &request).await {
            Ok(response) => return parse_ai_response(&response),
            Err(e) => {
                last_error = Some(e);
                if attempt < MAX_RETRIES {
                    tokio::time::sleep(Duration::from_secs(2u64.pow(attempt))).await;
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        AppError::ExternalServiceError("AI analysis failed after retries".to_string())
    }))
}

async fn send_request(
    config: &AiConfig,
    client: &Client,
    request: &AnthropicRequest,
) -> Result<String, AppError> {
    let response = client
        .post(ANTHROPIC_API_URL)
        .header("x-api-key", &config.api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(request)
        .send()
        .await
        .map_err(|e| AppError::ExternalServiceError(format!("API request failed: {}", e)))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(AppError::ExternalServiceError(format!(
            "API returned error {}: {}",
            status, error_text
        )));
    }

    let api_response: AnthropicResponse = response.json().await.map_err(|e| {
        AppError::ExternalServiceError(format!("Failed to parse API response: {}", e))
    })?;

    Ok(api_response
        .content
        .first()
        .map(|c| c.text.clone())
        .unwrap_or_default())
}

fn parse_ai_response(response_text: &str) -> Result<AiAnalysisResponse, AppError> {
    let trimmed = response_text.trim();
    let json_str = if trimmed.starts_with("```json") {
        trimmed
            .strip_prefix("```json")
            .and_then(|s| s.strip_suffix("```"))
            .unwrap_or(trimmed)
            .trim()
    } else if trimmed.starts_with("```") {
        trimmed
            .strip_prefix("```")
            .and_then(|s| s.strip_suffix("```"))
            .unwrap_or(trimmed)
            .trim()
    } else {
        trimmed
    };

    serde_json::from_str(json_str).map_err(|e| {
        AppError::ExternalServiceError(format!("Failed to parse AI JSON response: {}", e))
    })
}

async fn merge_chunk_results(
    config: &AiConfig,
    client: &Client,
    results: Vec<AiAnalysisResponse>,
) -> Result<AiAnalysisResponse, AppError> {
    let summaries: Vec<String> = results
        .iter()
        .enumerate()
        .map(|(i, r)| {
            format!(
                "Chunk {}: {} ({}  days, {} modules)",
                i + 1,
                r.program_title,
                r.estimated_total_days,
                r.modules.len()
            )
        })
        .collect();

    let merge_prompt = format!(
        "Given these separately analyzed curriculum sections, produce a unified program. \
         Deduplicate overlapping content, ensure day numbering is sequential, \
         resolve cross-chunk dependencies, and ensure logical module grouping.\n\n\
         Summaries:\n{}\n\n\
         Full results:\n{}",
        summaries.join("\n"),
        serde_json::to_string_pretty(&results).unwrap_or_default()
    );

    let system_prompt = get_system_prompt();
    let request = AnthropicRequest {
        model: MODEL.to_string(),
        max_tokens: MAX_TOKENS,
        temperature: TEMPERATURE,
        system: system_prompt,
        messages: vec![Message {
            role: "user".to_string(),
            content: merge_prompt,
        }],
    };

    let response = send_request(config, client, &request).await?;
    parse_ai_response(&response)
}

fn get_system_prompt() -> String {
    r#"You are a curriculum architect. Given educational content, you produce a structured learning plan as JSON.

Analyze the provided content and generate a complete learning program with the following structure:

{
  "program_title": "string",
  "program_description": "string",
  "estimated_total_days": number,
  "modules": [
    {
      "title": "string",
      "description": "string",
      "order_index": number,
      "color": "hex string",
      "days": [
        {
          "day_number": number,
          "title": "string — specific and actionable",
          "syntax_targets": "markdown — exact syntax/concepts to master",
          "implementation_brief": "markdown — what to build, specific deliverables",
          "files_to_create": "markdown — list of files the learner should create",
          "success_criteria": "markdown — measurable criteria for completion",
          "stretch_challenge": "markdown — optional advanced extension",
          "notes": "markdown — tips, gotchas, references",
          "estimated_minutes": number,
          "memory_rebuild_minutes": number,
          "checklist_items": [
            { "label": "string — specific task", "is_required": boolean }
          ],
          "quiz_questions": [
            {
              "question_text": "string",
              "question_type": "short_answer | multiple_choice | code_prompt | reflection",
              "correct_answer": "string",
              "options": ["string"] | [],
              "points": number,
              "time_limit_seconds": number
            }
          ],
          "concept_tags": [
            { "name": "string", "domain": "string" }
          ],
          "dependencies": [
            { "depends_on_day_number": number, "type": "prerequisite | recommended", "minimum_score": number }
          ]
        }
      ]
    }
  ]
}

Rules:
- Each day should take 45-120 minutes for a focused learner.
- Day titles must be specific (not "Learn CSS" but "CSS Grid Layout + Template Areas").
- Syntax targets must include exact code patterns the learner should memorize.
- Implementation briefs must describe a concrete build output, not vague exercises.
- Every day gets 3-8 checklist items that are individually verifiable.
- Every day gets 2-5 quiz questions covering the day's core concepts.
- Concept tags use granular names (not "CSS" but "$state rune", "grid-template-columns", "async/await").
- Dependencies should be set when a day requires knowledge from an earlier day.
- Group days into logical modules (5-10 days each).
- Assign module colors from this palette: #6366F1, #EC4899, #F59E0B, #22C55E, #3B82F6, #A855F7, #EF4444, #14B8A6.
- Stretch challenges should be genuinely harder, not just "do more of the same".
- Quiz questions should test understanding, not just recall.
- Respond with ONLY the JSON object. No markdown fences, no preamble."#.to_string()
}
