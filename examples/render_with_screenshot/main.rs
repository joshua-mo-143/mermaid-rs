use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
use mermaid_rs::Mermaid;

fn main() {
    let mermaid = Mermaid::new().unwrap();
    let res = mermaid
        .render_with_screenshot("graph TD;\nA[HTML Label] --> B[Regular Node];")
        .unwrap();

    std::fs::write("res.png", res).unwrap();
}
