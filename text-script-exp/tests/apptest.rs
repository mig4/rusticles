use anyhow::{Context, Result};
use assert_cmd::Command;
use predicates::prelude::*;
use std::path::{Path, PathBuf};

fn command() -> Result<Command> {
    Command::cargo_bin("text-script-exp").context("Could not get path to main command")
}

fn resource(p: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/resources").join(p)
}

#[test]
fn it_outputs_a_summary() -> Result<()> {
    let mut cmd = command()?;
    let fpath = resource("prometheus.resource-capacity.util.txt");

    cmd.arg(&fpath);
    cmd.assert()
        .success()
        .stdout(
            predicate::path::eq_file(resource(
                "prometheus.resource-capacity.old-new-comparison.txt"
            ))
        );

    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<()> {
    let mut cmd = command()?;
    let fpath = "file/doesnt/exist";

    cmd.arg(fpath);
    cmd.assert()
        .failure()
        .stderr(
            predicate::str::contains(fpath)
                .and(predicate::str::contains("Could not read input"))
        );

    Ok(())
}
