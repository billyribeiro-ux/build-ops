CREATE VIRTUAL TABLE IF NOT EXISTS search_index USING fts5(
    entity_id,
    entity_type,
    title,
    content,
    tags,
    tokenize='porter unicode61'
);
