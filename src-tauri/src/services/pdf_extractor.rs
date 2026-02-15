use crate::db::models::import::{
    CodeBlock, DocumentMetadata, ExtractedDocument, ExtractedSection,
};
use crate::error::AppError;
use pdf_extract::extract_text;
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub async fn extract_document(file_path: &str) -> Result<ExtractedDocument, AppError> {
    let path = Path::new(file_path);
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "pdf" => extract_pdf(file_path, &file_name).await,
        "md" | "markdown" => extract_markdown(file_path, &file_name).await,
        "txt" => extract_text_file(file_path, &file_name).await,
        _ => Err(AppError::ValidationError(format!(
            "Unsupported file type: {}",
            extension
        ))),
    }
}

async fn extract_pdf(file_path: &str, file_name: &str) -> Result<ExtractedDocument, AppError> {
    let text = extract_text(file_path).map_err(|e| {
        AppError::ExternalServiceError(format!("PDF extraction failed: {}", e))
    })?;

    let lines: Vec<&str> = text.lines().collect();
    let mut sections = Vec::new();
    let mut code_blocks = Vec::new();
    let mut current_section = String::new();
    let mut current_heading = "Introduction".to_string();
    let mut current_level = 1u8;
    let mut page_number = 1usize;
    let mut in_code_block = false;
    let mut code_content = String::new();
    let mut detected_languages = HashSet::new();

    for line in lines.iter() {
        if line.trim().is_empty() {
            continue;
        }

        if is_heading_heuristic(line) {
            if !current_section.is_empty() {
                let has_code = current_section.contains("```") || current_section.contains("    ");
                let has_list = current_section.contains("- ") || current_section.contains("* ");
                let complexity = estimate_complexity(&current_section, has_code);

                sections.push(ExtractedSection {
                    heading: current_heading.clone(),
                    level: current_level,
                    content: current_section.trim().to_string(),
                    page_number,
                    has_code,
                    has_list,
                    estimated_complexity: complexity,
                });
                current_section.clear();
            }

            current_heading = line.trim().to_string();
            current_level = detect_heading_level(line);
            page_number += 1;
            continue;
        }

        if line.starts_with("    ") || line.starts_with("\t") {
            if !in_code_block {
                in_code_block = true;
                code_content.clear();
            }
            code_content.push_str(line.trim());
            code_content.push('\n');
        } else if in_code_block {
            in_code_block = false;
            let language = detect_language(&code_content);
            if let Some(lang) = &language {
                detected_languages.insert(lang.clone());
            }
            code_blocks.push(CodeBlock {
                language,
                content: code_content.clone(),
                context_heading: current_heading.clone(),
                page_number,
            });
            code_content.clear();
        }

        current_section.push_str(line);
        current_section.push('\n');
    }

    if !current_section.is_empty() {
        let has_code = current_section.contains("```") || current_section.contains("    ");
        let has_list = current_section.contains("- ") || current_section.contains("* ");
        sections.push(ExtractedSection {
            heading: current_heading,
            level: current_level,
            content: current_section.trim().to_string(),
            page_number,
            has_code,
            has_list,
            estimated_complexity: estimate_complexity(&current_section, has_code),
        });
    }

    let word_count = text.split_whitespace().count();
    let detected_topics = extract_topics(&text);

    Ok(ExtractedDocument {
        file_name: file_name.to_string(),
        total_pages: page_number,
        raw_text: text,
        sections,
        code_blocks,
        metadata: DocumentMetadata {
            title: Some(file_name.to_string()),
            author: None,
            page_count: page_number,
            word_count,
            detected_languages: detected_languages.into_iter().collect(),
            detected_topics,
        },
    })
}

async fn extract_markdown(
    file_path: &str,
    file_name: &str,
) -> Result<ExtractedDocument, AppError> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| AppError::ExternalServiceError(format!("Failed to read file: {}", e)))?;

    let parser = Parser::new(&content);
    let mut sections = Vec::new();
    let mut code_blocks = Vec::new();
    let mut current_section = String::new();
    let mut current_heading = "Introduction".to_string();
    let mut current_level = 1u8;
    let mut page_number = 1usize;
    let mut detected_languages = HashSet::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                if !current_section.is_empty() {
                    let has_code = current_section.contains("```");
                    let has_list = current_section.contains("- ") || current_section.contains("* ");
                    sections.push(ExtractedSection {
                        heading: current_heading.clone(),
                        level: current_level,
                        content: current_section.trim().to_string(),
                        page_number,
                        has_code,
                        has_list,
                        estimated_complexity: estimate_complexity(&current_section, has_code),
                    });
                    current_section.clear();
                }
                current_level = level as u8;
            }
            Event::End(TagEnd::Heading(_)) => {
                page_number += 1;
            }
            Event::Text(text) => {
                if current_level > 0 && current_heading == "Introduction" {
                    current_heading = text.to_string();
                } else {
                    current_section.push_str(&text);
                }
            }
            Event::Code(code) => {
                current_section.push('`');
                current_section.push_str(&code);
                current_section.push('`');
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                let language = match kind {
                    pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                        let lang_str = lang.to_string();
                        detected_languages.insert(lang_str.clone());
                        Some(lang_str)
                    }
                    _ => None,
                };
                code_blocks.push(CodeBlock {
                    language,
                    content: String::new(),
                    context_heading: current_heading.clone(),
                    page_number,
                });
            }
            _ => {}
        }
    }

    if !current_section.is_empty() {
        let has_code = current_section.contains("```");
        let has_list = current_section.contains("- ") || current_section.contains("* ");
        sections.push(ExtractedSection {
            heading: current_heading,
            level: current_level,
            content: current_section.trim().to_string(),
            page_number,
            has_code,
            has_list,
            estimated_complexity: estimate_complexity(&current_section, has_code),
        });
    }

    let word_count = content.split_whitespace().count();
    let detected_topics = extract_topics(&content);

    Ok(ExtractedDocument {
        file_name: file_name.to_string(),
        total_pages: page_number,
        raw_text: content,
        sections,
        code_blocks,
        metadata: DocumentMetadata {
            title: Some(file_name.to_string()),
            author: None,
            page_count: page_number,
            word_count,
            detected_languages: detected_languages.into_iter().collect(),
            detected_topics,
        },
    })
}

async fn extract_text_file(
    file_path: &str,
    file_name: &str,
) -> Result<ExtractedDocument, AppError> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| AppError::ExternalServiceError(format!("Failed to read file: {}", e)))?;

    let lines: Vec<&str> = content.lines().collect();
    let mut sections = Vec::new();
    let mut current_section = String::new();
    let mut current_heading = "Introduction".to_string();
    let mut page_number = 1usize;

    for line in lines {
        if line.trim().is_empty() {
            if !current_section.is_empty() {
                let has_code = current_section.contains("    ");
                let has_list = current_section.contains("- ") || current_section.contains("* ");
                sections.push(ExtractedSection {
                    heading: current_heading.clone(),
                    level: 1,
                    content: current_section.trim().to_string(),
                    page_number,
                    has_code,
                    has_list,
                    estimated_complexity: estimate_complexity(&current_section, has_code),
                });
                current_section.clear();
                page_number += 1;
            }
            continue;
        }

        if line.chars().all(|c| c.is_uppercase() || c.is_whitespace() || c.is_numeric()) {
            current_heading = line.trim().to_string();
            continue;
        }

        current_section.push_str(line);
        current_section.push('\n');
    }

    if !current_section.is_empty() {
        let has_code = current_section.contains("    ");
        let has_list = current_section.contains("- ") || current_section.contains("* ");
        sections.push(ExtractedSection {
            heading: current_heading,
            level: 1,
            content: current_section.trim().to_string(),
            page_number,
            has_code,
            has_list,
            estimated_complexity: estimate_complexity(&current_section, has_code),
        });
    }

    let word_count = content.split_whitespace().count();
    let detected_topics = extract_topics(&content);

    Ok(ExtractedDocument {
        file_name: file_name.to_string(),
        total_pages: page_number,
        raw_text: content,
        sections,
        code_blocks: Vec::new(),
        metadata: DocumentMetadata {
            title: Some(file_name.to_string()),
            author: None,
            page_count: page_number,
            word_count,
            detected_languages: Vec::new(),
            detected_topics,
        },
    })
}

fn is_heading_heuristic(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return false;
    }

    trimmed.chars().all(|c| c.is_uppercase() || c.is_whitespace() || c.is_numeric())
        || trimmed.starts_with("Chapter ")
        || trimmed.starts_with("Section ")
        || trimmed.starts_with("Part ")
        || (trimmed.len() < 60 && trimmed.ends_with(':'))
}

fn detect_heading_level(line: &str) -> u8 {
    if line.starts_with("# ") {
        1
    } else if line.starts_with("## ") {
        2
    } else if line.starts_with("### ") {
        3
    } else if line.chars().all(|c| c.is_uppercase() || c.is_whitespace()) {
        1
    } else {
        2
    }
}

fn detect_language(code: &str) -> Option<String> {
    if code.contains("fn ") || code.contains("impl ") || code.contains("pub ") {
        Some("rust".to_string())
    } else if code.contains("const ") || code.contains("let ") || code.contains("function ") {
        Some("javascript".to_string())
    } else if code.contains("def ") || code.contains("import ") {
        Some("python".to_string())
    } else if code.contains("<script>") || code.contains("</script>") {
        Some("svelte".to_string())
    } else if code.contains("{") && code.contains("}") && code.contains(":") {
        Some("css".to_string())
    } else {
        None
    }
}

fn estimate_complexity(content: &str, has_code: bool) -> u8 {
    let mut score = 1u8;

    if has_code {
        score += 2;
    }

    if content.len() > 2000 {
        score += 1;
    }

    if content.len() > 5000 {
        score += 1;
    }

    let technical_keywords = [
        "algorithm",
        "implementation",
        "architecture",
        "async",
        "await",
        "closure",
        "generic",
        "trait",
        "interface",
    ];

    for keyword in &technical_keywords {
        if content.to_lowercase().contains(keyword) {
            score += 1;
            break;
        }
    }

    score.min(5)
}

fn extract_topics(text: &str) -> Vec<String> {
    let mut topics = HashSet::new();
    let keywords = [
        "svelte",
        "react",
        "vue",
        "typescript",
        "javascript",
        "rust",
        "python",
        "css",
        "html",
        "api",
        "database",
        "authentication",
        "testing",
        "deployment",
    ];

    let lower_text = text.to_lowercase();
    for keyword in &keywords {
        if lower_text.contains(keyword) {
            topics.insert(keyword.to_string());
        }
    }

    topics.into_iter().collect()
}
