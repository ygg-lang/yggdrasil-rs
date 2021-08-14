use super::*;

pub static COMPLETE_ESCAPES: SyncLazy<Vec<CompletionItem>> = SyncLazy::new(|| complete_escapes());

pub fn complete_escapes() -> Vec<CompletionItem> {
    let mut out = vec![];
    out.extend(load_md_doc(include_str!("escapes_args.md")).iter().map(|doc| build_command(doc, true)));
    out.extend(load_md_doc(include_str!("escapes.md")).iter().map(|doc| build_command(doc, false)));
    return out;
}

pub fn build_command(doc: &DocumentString, args: bool) -> CompletionItem {
    let cmd = doc.cmd.to_owned();
    let short = doc.short.to_owned();
    let doc = MarkupContent { kind: MarkupKind::Markdown, value: doc.long.to_owned() };
    let insert_text = match args {
        true => format!("{}{{$1}}", cmd),
        false => cmd.clone(),
    };
    CompletionItem {
        label: format!("{}", cmd),
        label_details: None,
        kind: Some(CompletionItemKind::KEYWORD),
        detail: Some(short),
        documentation: Some(Documentation::MarkupContent(doc)),
        sort_text: None,
        filter_text: None,
        insert_text: Some(insert_text),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        insert_text_mode: None,
        ..CompletionItem::default()
    }
}
