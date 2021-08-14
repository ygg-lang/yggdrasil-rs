use super::*;

pub static COMPLETE_KEYWORDS: SyncLazy<Vec<CompletionItem>> = SyncLazy::new(|| complete_keywords());

pub fn complete_keywords() -> Vec<CompletionItem> {
    let parsed = load_md_doc(include_str!("keywords.md"));
    parsed.iter().map(build_command).collect()
}

pub fn build_command(doc: &DocumentString) -> CompletionItem {
    let cmd = doc.cmd.to_owned();
    let short = doc.short.to_owned();
    let doc = MarkupContent { kind: MarkupKind::Markdown, value: doc.long.to_owned() };
    CompletionItem {
        label: cmd.clone(),
        label_details: None,
        kind: Some(CompletionItemKind::KEYWORD),
        detail: Some(short),
        documentation: Some(Documentation::MarkupContent(doc)),
        sort_text: Some(format!("0{}", cmd)),
        filter_text: None,
        insert_text: Some(format!("{} ", cmd)),
        insert_text_format: None,
        insert_text_mode: None,
        ..CompletionItem::default()
    }
}
