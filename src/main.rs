use web_view::*;

fn main() {
    web_view::builder()
        .title("Minimal webview example")
        .content(Content::Url("https://www.ferrari.com"))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
