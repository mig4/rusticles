use anyhow::Result;  // For nicer error handling
use assert_cmd::prelude::*;  // Add methods on commands
use predicates::prelude::*;  // Used for writing assertions
use std::io::Write;  // For writing to tempfile
use std::process::Command;  // Run programs
use tempfile::NamedTempFile;  // For creating files to search through

#[test]
fn file_doesnt_exist() -> Result<()> {
    let mut cmd = Command::cargo_bin("grrs")?;
    let file = "test/file/doesnt/exist";

    cmd.arg("foobar").arg(file);
    cmd.assert()
        .failure()
        .stderr(
            predicate::str::contains(file)
            .and(predicate::str::contains("No such file or directory"))
        );

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<()> {
    let mut cmd = Command::cargo_bin("grrs")?;
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::eq("A test\nAnother test\n"));

    Ok(())
}

#[test]
fn empty_pattern() -> Result<()> {
    let mut cmd = Command::cargo_bin("grrs")?;
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nAnother test")?;

    cmd.arg("").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::eq("A test\nAnother test\n"));

    Ok(())
}
