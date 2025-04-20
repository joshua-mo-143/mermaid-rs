use std::{error::Error, sync::Arc};

use error::CompileError;
use escape_string::escape;
use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption, Browser, Tab};
use unescape::unescape;

mod error;

/// The Mermaid struct holds the embedded Chromium instance that is used to render Mermaid
/// diagrams
#[derive(Clone)]
pub struct Mermaid {
    _browser: Browser,
    tab: Arc<Tab>,
}

impl Mermaid {
    /// Initializes Mermaid
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let browser = Browser::default()?;
        let mermaid_js = include_str!("../payload/mermaid.min.js");
        let html_payload = include_str!("../payload/index.html");

        let tab = browser.new_tab()?;
        tab.navigate_to(&format!("data:text/html;charset=utf-8,{}", html_payload))?;
        if let Err(e) = tab.evaluate(mermaid_js, true) {
            println!("Error while evaluating: {e}");
        };

        Ok(Self {
            _browser: browser,
            tab,
        })
    }

    /// Renders a diagram
    ///
    /// # Example:
    /// ```
    /// let mermaid = Mermaid::new();
    /// let svg = mermaid.render("graph TB\na-->b").expect("Unable to render!");
    /// ```
    pub fn render(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let data = self
            .tab
            .evaluate(&format!("render('{}')", escape(input)), true)?;
        let string = data.value.unwrap_or_default().to_string();
        let slice = unescape(string.trim_matches('"')).unwrap_or_default();

        if slice == "null" {
            return Err(Box::new(CompileError));
        }

        Ok(slice.to_string())
    }

    /// Renders a diagram and takes a screenshot
    ///
    /// # Example:
    /// ```
    /// let mermaid = Mermaid::new();
    /// let svg = mermaid.render("graph TB\na-->b").expect("Unable to render!");
    /// ```
    pub fn render_with_screenshot(&self, input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let _ = self
            .tab
            .evaluate(&format!("render('{}')", escape(input)), true)?;

        let res = self
            .tab
            .find_element("#div")
            .unwrap()
            .capture_screenshot(CaptureScreenshotFormatOption::Png)
            .unwrap();

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_mermaid_instance_without_crashing() {
        let mermaid = Mermaid::new();
        assert!(mermaid.is_ok());
    }

    #[test]
    fn render_mermaid() {
        let mermaid = Mermaid::new().unwrap();
        let rendered = mermaid.render("graph TB\na-->b");
        assert!(rendered.is_ok());
        // TODO: Perform visual image comparison
        assert!(rendered.unwrap().starts_with("<svg"));
    }

    #[test]
    fn syntax_error() {
        let mermaid = Mermaid::new().unwrap();
        let rendered = mermaid.render("grph TB\na-->b");
        assert!(rendered.is_err());
    }
}
