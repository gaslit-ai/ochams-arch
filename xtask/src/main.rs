use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Output};

use ochams_fixtures::{
    CommandKind, display_fixture, expected_stderr, expected_stdout, fixture_paths,
    read_expected_exit, read_required,
};
use serde_json::Value;

fn main() -> ExitCode {
    match run(env::args_os().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(1)
        }
    }
}

fn run(args: Vec<OsString>) -> Result<(), String> {
    match args.as_slice() {
        [command] if command == "verify-fixtures" => {
            let workspace = workspace_root();
            let ochams = build_ochams(&workspace, Mode::Verify)?;
            verify_fixtures(&workspace, &ochams)
        }
        [command] if command == "regenerate-fixtures" => {
            let workspace = workspace_root();
            let ochams = build_ochams(&workspace, Mode::Regenerate)?;
            regenerate_fixtures(&workspace, &ochams)
        }
        _ => Err("usage: cargo verify-fixtures | cargo regenerate-fixtures".to_owned()),
    }
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    Verify,
    Regenerate,
}

fn verify_fixtures(workspace: &Path, ochams: &Path) -> Result<(), String> {
    exercise_fixtures(workspace, ochams, Mode::Verify)
}

fn regenerate_fixtures(workspace: &Path, ochams: &Path) -> Result<(), String> {
    exercise_fixtures(workspace, ochams, Mode::Regenerate)
}

fn exercise_fixtures(workspace: &Path, ochams: &Path, mode: Mode) -> Result<(), String> {
    let mut exercised = Vec::new();
    for fixture in fixture_paths(workspace)? {
        for kind in fixture.commands {
            exercise_fixture_command(&fixture.path, kind, ochams, mode)?;
            exercised.push(format!(
                "{}:{}",
                display_fixture(&fixture.path),
                kind.name()
            ));
        }
    }

    if exercised.is_empty() {
        return Err("no golden command fixtures were exercised".to_owned());
    }

    match mode {
        Mode::Verify => println!("verified {} fixture commands", exercised.len()),
        Mode::Regenerate => println!("regenerated {} fixture commands", exercised.len()),
    }
    Ok(())
}

fn exercise_fixture_command(
    fixture: &Path,
    kind: CommandKind,
    ochams: &Path,
    mode: Mode,
) -> Result<(), String> {
    let output = CommandOutput::from(run_ochams(fixture, kind, ochams)?);
    match mode {
        Mode::Verify => verify_output(fixture, kind, output),
        Mode::Regenerate => regenerate_output(fixture, kind, output),
    }
}

#[derive(Debug)]
struct CommandOutput {
    exit_code: i32,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

impl From<Output> for CommandOutput {
    fn from(output: Output) -> Self {
        Self {
            exit_code: output.status.code().unwrap_or(1),
            stdout: output.stdout,
            stderr: output.stderr,
        }
    }
}

fn run_ochams(fixture: &Path, kind: CommandKind, ochams: &Path) -> Result<Output, String> {
    let repo = fixture.join("repo");
    let mut command = Command::new(ochams);
    match kind {
        CommandKind::Check => {
            command.arg("check").arg(repo);
        }
        CommandKind::GraphJson => {
            command.arg("graph").arg(repo).arg("--format").arg("json");
        }
        CommandKind::Query => {
            let symbol = read_required(
                fixture,
                kind.query_symbol_path()
                    .expect("query command requires query symbol"),
            )?;
            command.arg("query").arg(repo).arg(symbol.trim());
        }
    }

    command
        .output()
        .map_err(|error| format!("could not run {}: {error}", ochams.display()))
}

fn verify_output(fixture: &Path, kind: CommandKind, output: CommandOutput) -> Result<(), String> {
    let expected_exit = read_expected_exit(fixture, kind)?;
    let expected_stdout = expected_stdout(fixture, kind)?;
    let expected_stderr = expected_stderr(fixture, kind)?;
    let actual_stdout = String::from_utf8(output.stdout).map_err(|error| {
        format!(
            "{} {} stdout is not UTF-8: {error}",
            display_fixture(fixture),
            kind.name()
        )
    })?;
    let actual_stderr = String::from_utf8(output.stderr).map_err(|error| {
        format!(
            "{} {} stderr is not UTF-8: {error}",
            display_fixture(fixture),
            kind.name()
        )
    })?;
    let actual_exit = output.exit_code;

    let mut failures = Vec::new();
    if actual_exit != expected_exit {
        failures.push(format!("exit expected {expected_exit}, got {actual_exit}"));
    }
    if actual_stdout != expected_stdout {
        failures.push(format!(
            "stdout drift\nexpected:\n{expected_stdout}\nactual:\n{actual_stdout}"
        ));
    }
    if actual_stderr != expected_stderr {
        failures.push(format!(
            "stderr drift\nexpected:\n{expected_stderr}\nactual:\n{actual_stderr}"
        ));
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{} {} fixture drift:\n{}",
            display_fixture(fixture),
            kind.name(),
            failures.join("\n")
        ))
    }
}

fn regenerate_output(
    fixture: &Path,
    kind: CommandKind,
    output: CommandOutput,
) -> Result<(), String> {
    write_expected(
        fixture,
        kind.expected_exit_path(),
        &format!("{}\n", output.exit_code),
    )?;
    write_stream(
        fixture,
        kind.expected_stderr_path(),
        kind,
        "stderr",
        &output.stderr,
    )?;
    write_stdout(fixture, kind, &output.stdout)?;
    Ok(())
}

fn write_stdout(fixture: &Path, kind: CommandKind, stdout: &[u8]) -> Result<(), String> {
    for rel_path in kind.stdout_cleanup_paths() {
        remove_if_exists(fixture.join(rel_path))?;
    }

    if let Some(rel_path) = kind.expected_stdout_write_path(stdout) {
        write_expected(
            fixture,
            rel_path,
            bytes_as_utf8(fixture, kind, "stdout", stdout)?,
        )?;
    }
    Ok(())
}

fn write_stream(
    fixture: &Path,
    rel_path: &str,
    kind: CommandKind,
    stream_name: &str,
    bytes: &[u8],
) -> Result<(), String> {
    if bytes.is_empty() {
        remove_if_exists(fixture.join(rel_path))?;
    } else {
        write_expected(
            fixture,
            rel_path,
            bytes_as_utf8(fixture, kind, stream_name, bytes)?,
        )?;
    }
    Ok(())
}

fn bytes_as_utf8<'a>(
    fixture: &Path,
    kind: CommandKind,
    stream: &str,
    bytes: &'a [u8],
) -> Result<&'a str, String> {
    std::str::from_utf8(bytes).map_err(|error| {
        format!(
            "{} {} {stream} is not UTF-8: {error}",
            display_fixture(fixture),
            kind.name()
        )
    })
}

fn write_expected(fixture: &Path, rel_path: &str, content: &str) -> Result<(), String> {
    fs::write(fixture.join(rel_path), content).map_err(|error| {
        format!(
            "could not write fixture file {}: {error}",
            fixture.join(rel_path).display()
        )
    })
}

fn remove_if_exists(path: PathBuf) -> Result<(), String> {
    match fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("could not remove {}: {error}", path.display())),
    }
}

fn build_ochams(workspace: &Path, mode: Mode) -> Result<PathBuf, String> {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
    let mut command = Command::new(cargo);
    command
        .current_dir(workspace)
        .arg("build")
        .arg("-p")
        .arg("ochams-cli")
        .arg("--bin")
        .arg("ochams")
        .arg("--message-format=json-render-diagnostics");
    if matches!(mode, Mode::Verify) {
        command.arg("--locked");
    }

    let output = command
        .output()
        .map_err(|error| format!("could not run cargo build for ochams: {error}"))?;

    if !output.status.success() {
        return Err(format!(
            "cargo build for ochams failed with {}\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8(output.stdout)
        .map_err(|error| format!("cargo build output was not UTF-8: {error}"))?;
    find_ochams_executable(workspace, &stdout)
}

fn find_ochams_executable(workspace: &Path, stdout: &str) -> Result<PathBuf, String> {
    let expected_manifest = workspace.join("crates/ochams-cli/Cargo.toml");
    let mut executable = None;

    for (line_index, line) in stdout.lines().enumerate() {
        if !line.trim_start().starts_with('{') {
            continue;
        }

        let message = serde_json::from_str::<Value>(line).map_err(|error| {
            format!(
                "could not parse cargo JSON message at stdout line {}: {error}",
                line_index + 1
            )
        })?;

        if message.get("reason").and_then(Value::as_str) != Some("compiler-artifact") {
            continue;
        }

        let Some(target) = message.get("target") else {
            continue;
        };
        let target_name = target.get("name").and_then(Value::as_str);
        let is_bin = target
            .get("kind")
            .and_then(Value::as_array)
            .is_some_and(|kinds| kinds.iter().any(|kind| kind.as_str() == Some("bin")));
        let manifest_matches = message
            .get("manifest_path")
            .and_then(Value::as_str)
            .is_some_and(|manifest| Path::new(manifest) == expected_manifest);

        if target_name == Some("ochams") && is_bin && manifest_matches {
            if let Some(path) = message.get("executable").and_then(Value::as_str) {
                executable = Some(PathBuf::from(path));
            }
        }
    }

    executable.ok_or_else(|| "cargo build did not report an ochams executable artifact".to_owned())
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask is inside the workspace root")
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

    #[test]
    fn cargo_artifact_parser_uses_reported_executable_path() {
        let workspace = temp_dir("cargo_artifact_parser_uses_reported_executable_path");
        let manifest = workspace.join("crates/ochams-cli/Cargo.toml");
        let executable = workspace.join("custom-target/aarch64/debug/ochams");
        let stdout = serde_json::json!({
            "reason": "compiler-artifact",
            "manifest_path": manifest,
            "target": {
                "kind": ["bin"],
                "name": "ochams"
            },
            "executable": executable
        })
        .to_string();

        assert_eq!(
            find_ochams_executable(&workspace, &stdout).expect("executable"),
            workspace.join("custom-target/aarch64/debug/ochams")
        );
    }

    #[test]
    fn cargo_artifact_parser_rejects_missing_executable() {
        let workspace = temp_dir("cargo_artifact_parser_rejects_missing_executable");
        let stdout = serde_json::json!({
            "reason": "compiler-artifact",
            "manifest_path": workspace.join("crates/ochams-core/Cargo.toml"),
            "target": {
                "kind": ["lib"],
                "name": "ochams_core"
            },
            "executable": null
        })
        .to_string();

        assert!(
            find_ochams_executable(&workspace, &stdout)
                .expect_err("missing executable")
                .contains("did not report an ochams executable")
        );
    }

    #[test]
    fn regeneration_rewrites_expected_files_and_removes_empty_stream_files() {
        let fixture = temp_dir("regeneration_rewrites_expected_files");
        write(&fixture, "expected.graph.stdout", "stale\n");
        write(&fixture, "expected.graph.stdout.json", "old\n");
        write(&fixture, "expected.graph.stderr", "old stderr\n");

        regenerate_output(
            &fixture,
            CommandKind::GraphJson,
            CommandOutput {
                exit_code: 0,
                stdout: b"{\n}\n".to_vec(),
                stderr: Vec::new(),
            },
        )
        .expect("regenerate");

        assert_eq!(read(&fixture, "expected.graph.exit"), "0\n");
        assert_eq!(read(&fixture, "expected.graph.stdout.json"), "{\n}\n");
        assert!(!fixture.join("expected.graph.stdout").exists());
        assert!(!fixture.join("expected.graph.stderr").exists());
    }

    fn temp_dir(name: &str) -> PathBuf {
        let nonce = TEMP_COUNTER.fetch_add(1, Ordering::SeqCst);
        let path =
            std::env::temp_dir().join(format!("ochams-xtask-{name}-{}-{nonce}", process::id()));
        match fs::remove_dir_all(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(error) => panic!("could not clear {}: {error}", path.display()),
        }
        fs::create_dir_all(&path).expect("temp dir");
        path
    }

    fn write(root: &Path, rel_path: &str, content: &str) {
        let path = root.join(rel_path);
        fs::create_dir_all(path.parent().expect("parent")).expect("parent directory");
        fs::write(path, content).expect("write");
    }

    fn read(root: &Path, rel_path: &str) -> String {
        fs::read_to_string(root.join(rel_path)).expect("read")
    }
}
