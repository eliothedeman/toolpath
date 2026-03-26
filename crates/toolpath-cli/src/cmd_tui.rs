use std::path::PathBuf;

use anyhow::{Context, Result};
use toolpath::v1;
use toolpath_tui::{TuiConfig, TuiMode};

pub fn run_view(input: PathBuf) -> Result<()> {
    run_tui(input, TuiMode::View)
}

pub fn run_redact(input: PathBuf) -> Result<()> {
    run_tui(input, TuiMode::Redact)
}

fn run_tui(input: PathBuf, mode: TuiMode) -> Result<()> {
    let json = if input.to_str() == Some("-") {
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf
    } else {
        std::fs::read_to_string(&input).with_context(|| format!("reading {}", input.display()))?
    };

    let doc = v1::Document::from_json(&json).context("parsing Toolpath document")?;
    let path = match doc {
        v1::Document::Path(p) => p,
        _ => anyhow::bail!("view/redact requires a Path document"),
    };

    let config = TuiConfig {
        app_name: "path".to_string(),
    };

    let result = toolpath_tui::run(path, mode, config)?;
    if let Some(json) = result {
        println!("{json}");
    }
    Ok(())
}
