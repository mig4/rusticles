use anyhow::{Context, Result};
use std::io::prelude::*;

pub fn find_matches(content: impl BufRead, pattern: &str, mut writer: impl Write) -> Result<()> {
    for line in content.lines() {
        let line = line.context("Could not read line")?;
        if line.contains(pattern) {
            writeln!(writer, "{}", line).context("Could not output line")?;
        }
    }
    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();

    find_matches(
        std::io::Cursor::new("lorem ipsum\ndolor sit amet"),
        "lorem",
        &mut result,
    ).expect("find_matches failed unexpectedly");
    assert_eq!(result, b"lorem ipsum\n");
}
