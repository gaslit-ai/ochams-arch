use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use ochams_fixtures::{
    CommandKind, command_args, expected_stderr, expected_stdout, fixture_paths, read_expected_exit,
};

#[test]
fn golden_command_fixtures() {
    let mut exercised = Vec::new();
    for fixture in fixture_paths(&workspace_root()).expect("fixture paths") {
        for kind in fixture.commands {
            assert_fixture_command(&fixture.path, kind);
            exercised.push(format!(
                "{}:{}",
                fixture.path.file_name().unwrap().to_string_lossy(),
                kind.name()
            ));
        }
    }
    assert!(
        !exercised.is_empty(),
        "no golden command fixtures were exercised"
    );
}

#[test]
fn argument_errors_use_diagnostic_stderr() {
    let cases = [
        (
            vec!["graph", ".", "--format", "yaml"],
            "OCH020 unsupported graph format `yaml`\n",
        ),
        (
            vec!["graph", "."],
            "OCH020 usage: ochams check <root> | ochams graph <root> --format json | ochams query <root> <symbol>\n",
        ),
        (
            vec!["nope", "."],
            "OCH020 usage: ochams check <root> | ochams graph <root> --format json | ochams query <root> <symbol>\n",
        ),
        (
            vec!["check", ".", "extra"],
            "OCH020 usage: ochams check <root> | ochams graph <root> --format json | ochams query <root> <symbol>\n",
        ),
        (
            vec!["query", "."],
            "OCH020 usage: ochams check <root> | ochams graph <root> --format json | ochams query <root> <symbol>\n",
        ),
    ];

    for (args, expected_stderr) in cases {
        let output = ochams().args(args).output().expect("command");
        assert_output(output, 1, "", expected_stderr);
    }
}

fn assert_fixture_command(fixture: &Path, kind: CommandKind) {
    let repo = fixture.join("repo");
    let expected_exit = read_expected_exit(fixture, kind).expect("expected exit");
    let expected_stdout = expected_stdout(fixture, kind).expect("expected stdout");
    let expected_stderr = expected_stderr(fixture, kind).expect("expected stderr");

    let output = ochams()
        .args(command_args(fixture, kind, &repo).expect("command args"))
        .output()
        .expect("fixture command");

    assert_output(output, expected_exit, &expected_stdout, &expected_stderr);
}

fn assert_output(output: Output, expected_exit: i32, expected_stdout: &str, expected_stderr: &str) {
    assert_eq!(output.status.code().unwrap_or(1), expected_exit);
    assert_eq!(
        String::from_utf8(output.stdout).expect("stdout"),
        expected_stdout
    );
    assert_eq!(
        String::from_utf8(output.stderr).expect("stderr"),
        expected_stderr
    );
}

fn ochams() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ochams"))
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}
