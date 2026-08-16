/// Output formatting utilities.
#[derive(Clone, Debug, Default)]
pub struct CliOutput;

impl CliOutput {
    pub fn success(message: &str) -> String {
        format!("✓ {}", message)
    }

    pub fn error(message: &str) -> String {
        format!("✗ {}", message)
    }

    pub fn info(message: &str) -> String {
        format!("ℹ {}", message)
    }

    pub fn table_header(headers: &[&str]) -> String {
        headers.join(" | ")
    }

    pub fn table_row(cells: &[&str]) -> String {
        cells.join(" | ")
    }

    pub fn code_block(content: &str) -> String {
        format!("```\n{}\n```", content)
    }
}
