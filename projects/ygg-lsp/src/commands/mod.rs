use lspower::{lsp::*, Client};
use serde_json::Value;
use std::{collections::HashSet, lazy::SyncLazy};

pub fn server_commands() -> ExecuteCommandOptions {
    let commands = SERVER_COMMANDS.iter().map(|s| s.to_string()).collect();
    ExecuteCommandOptions { commands, work_done_progress_options: Default::default() }
}

static SERVER_COMMANDS: SyncLazy<HashSet<&'static str>> = SyncLazy::new(|| {
    let mut s = HashSet::new();
    s.insert("ygg.inner.convert.json");
    s.insert("arc.inner.get-web-view");
    s.insert("arc.inner.post-math-svg");
    s.insert("arc.rawPaste");
    s.insert("arc.image.save2local");
    return s;
});

pub async fn command_provider(p: ExecuteCommandParams, c: &Client) -> Option<Value> {
    // c.show_message(MessageType::Log, format!("{:#?}", p.command)).await;
    // c.show_message(MessageType::Log, format!("{:#?}", p.arguments)).await;
    match p.command.as_ref() {
        "notedown.inner.get-web-view" => get_web_view().await,
        "notedown.inner.request-math-svg" => request_math_svg(c).await,
        _ => {
            let err = format!("Unknown command: {}", p.command);
            c.show_message(MessageType::ERROR, err).await;
            return None;
        }
    }
}

async fn request_math_svg(c: &Client) -> Option<Value> {
    let _ = c;
    None
}

async fn get_web_view() -> Option<Value> {
    let head = r#"
        <meta charset="utf-8"/>
        <title>Notedown editor</title>

        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/font-awesome@4.7.0/css/font-awesome.min.css"/>
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/aplayer/dist/APlayer.min.css">

        <script src="https://cdn.jsdelivr.net/npm/aplayer/dist/APlayer.min.js"></script>
        <script src="https://cdn.jsdelivr.net/npm/meting@2/dist/Meting.min.js"></script>
    "#;
    let html = format!(
        r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>{head}</head>
    <body>body</body>
    </html>
    "#,
        head = head
    );
    return Some(Value::String(html));
}
