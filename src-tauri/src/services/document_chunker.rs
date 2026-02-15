use crate::db::models::import::{ChunkStrategy, ChunkedDocument, DocumentChunk, ExtractedDocument};
use crate::error::AppError;
use tiktoken_rs::cl100k_base;

const SINGLE_PASS_LIMIT: usize = 150_000;
const SECTION_CHUNK_LIMIT: usize = 100_000;
const MULTI_PASS_LIMIT: usize = 80_000;

pub fn chunk_document(doc: &ExtractedDocument) -> Result<ChunkedDocument, AppError> {
    let bpe = cl100k_base().map_err(|e| {
        AppError::ExternalServiceError(format!("Failed to initialize tokenizer: {}", e))
    })?;

    let total_tokens = bpe.encode_with_special_tokens(&doc.raw_text).len();

    if total_tokens < SINGLE_PASS_LIMIT {
        return Ok(ChunkedDocument {
            chunks: vec![DocumentChunk {
                index: 0,
                content: doc.raw_text.clone(),
                token_count: total_tokens,
                section_refs: (0..doc.sections.len()).collect(),
                is_continuation: false,
            }],
            total_tokens,
            chunk_strategy: ChunkStrategy::SinglePass,
        });
    }

    if total_tokens < 500_000 {
        return chunk_by_sections(doc, &bpe, SECTION_CHUNK_LIMIT);
    }

    chunk_multi_pass(doc, &bpe, MULTI_PASS_LIMIT)
}

fn chunk_by_sections(
    doc: &ExtractedDocument,
    bpe: &tiktoken_rs::CoreBPE,
    max_tokens: usize,
) -> Result<ChunkedDocument, AppError> {
    let mut chunks = Vec::new();
    let mut current_content = String::new();
    let mut current_sections = Vec::new();
    let mut current_tokens = 0;
    let mut chunk_index = 0;

    for (idx, section) in doc.sections.iter().enumerate() {
        let section_text = format!("\n\n## {}\n\n{}", section.heading, section.content);
        let section_tokens = bpe.encode_with_special_tokens(&section_text).len();

        if current_tokens + section_tokens > max_tokens && !current_content.is_empty() {
            chunks.push(DocumentChunk {
                index: chunk_index,
                content: current_content.clone(),
                token_count: current_tokens,
                section_refs: current_sections.clone(),
                is_continuation: chunk_index > 0,
            });

            chunk_index += 1;
            current_content.clear();
            current_sections.clear();
            current_tokens = 0;
        }

        current_content.push_str(&section_text);
        current_sections.push(idx);
        current_tokens += section_tokens;
    }

    if !current_content.is_empty() {
        chunks.push(DocumentChunk {
            index: chunk_index,
            content: current_content,
            token_count: current_tokens,
            section_refs: current_sections,
            is_continuation: chunk_index > 0,
        });
    }

    let total_tokens = chunks.iter().map(|c| c.token_count).sum();

    Ok(ChunkedDocument {
        chunks,
        total_tokens,
        chunk_strategy: ChunkStrategy::SectionBased,
    })
}

fn chunk_multi_pass(
    doc: &ExtractedDocument,
    bpe: &tiktoken_rs::CoreBPE,
    max_tokens: usize,
) -> Result<ChunkedDocument, AppError> {
    let mut chunks = Vec::new();
    let mut current_content = String::new();
    let mut current_sections = Vec::new();
    let mut current_tokens = 0;
    let mut chunk_index = 0;
    let mut last_paragraph = String::new();

    for (idx, section) in doc.sections.iter().enumerate() {
        let section_text = format!("\n\n## {}\n\n{}", section.heading, section.content);
        let section_tokens = bpe.encode_with_special_tokens(&section_text).len();

        if current_tokens + section_tokens > max_tokens && !current_content.is_empty() {
            let preamble = if chunk_index > 0 {
                format!(
                    "This is chunk {}/{} of document '{}'. Previous context: {}\n\n",
                    chunk_index + 1,
                    "TBD",
                    doc.file_name,
                    last_paragraph
                )
            } else {
                String::new()
            };

            let full_content = format!("{}{}", preamble, current_content);
            let full_tokens = bpe.encode_with_special_tokens(&full_content).len();

            chunks.push(DocumentChunk {
                index: chunk_index,
                content: full_content,
                token_count: full_tokens,
                section_refs: current_sections.clone(),
                is_continuation: chunk_index > 0,
            });

            last_paragraph = extract_last_paragraph(&current_content);
            chunk_index += 1;
            current_content.clear();
            current_sections.clear();
            current_tokens = 0;
        }

        current_content.push_str(&section_text);
        current_sections.push(idx);
        current_tokens += section_tokens;
    }

    if !current_content.is_empty() {
        let preamble = if chunk_index > 0 {
            format!(
                "This is chunk {}/{} of document '{}'. Previous context: {}\n\n",
                chunk_index + 1,
                chunk_index + 1,
                doc.file_name,
                last_paragraph
            )
        } else {
            String::new()
        };

        let full_content = format!("{}{}", preamble, current_content);
        let full_tokens = bpe.encode_with_special_tokens(&full_content).len();

        chunks.push(DocumentChunk {
            index: chunk_index,
            content: full_content,
            token_count: full_tokens,
            section_refs: current_sections,
            is_continuation: chunk_index > 0,
        });
    }

    let total = chunks.len();
    for chunk in &mut chunks {
        chunk.content = chunk.content.replace("TBD", &total.to_string());
    }

    let total_tokens = chunks.iter().map(|c| c.token_count).sum();

    Ok(ChunkedDocument {
        chunks,
        total_tokens,
        chunk_strategy: ChunkStrategy::MultiPass,
    })
}

fn extract_last_paragraph(content: &str) -> String {
    content
        .trim()
        .lines()
        .rev()
        .take(3)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(200)
        .collect()
}

pub fn merge_multi_file_content(docs: Vec<ExtractedDocument>) -> ExtractedDocument {
    let mut merged_text = String::new();
    let mut merged_sections = Vec::new();
    let mut merged_code_blocks = Vec::new();
    let mut total_pages = 0;
    let mut total_words = 0;
    let mut all_languages = Vec::new();
    let mut all_topics = Vec::new();

    for doc in docs {
        merged_text.push_str(&format!(
            "\n\n=== FILE: {} ({} pages) ===\n\n",
            doc.file_name, doc.total_pages
        ));
        merged_text.push_str(&doc.raw_text);

        merged_sections.extend(doc.sections);
        merged_code_blocks.extend(doc.code_blocks);
        total_pages += doc.total_pages;
        total_words += doc.metadata.word_count;
        all_languages.extend(doc.metadata.detected_languages);
        all_topics.extend(doc.metadata.detected_topics);
    }

    all_languages.sort();
    all_languages.dedup();
    all_topics.sort();
    all_topics.dedup();

    ExtractedDocument {
        file_name: "Multi-file Import".to_string(),
        total_pages,
        raw_text: merged_text,
        sections: merged_sections,
        code_blocks: merged_code_blocks,
        metadata: crate::db::models::import::DocumentMetadata {
            title: Some("Multi-file Import".to_string()),
            author: None,
            page_count: total_pages,
            word_count: total_words,
            detected_languages: all_languages,
            detected_topics: all_topics,
        },
    }
}
