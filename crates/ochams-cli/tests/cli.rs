use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

#[test]
fn golden_command_fixtures() {
    let mut exercised = Vec::new();
    for fixture in fixture_paths() {
        for kind in CommandKind::ALL {
            if fixture
                .join(format!("expected.{}.exit", kind.name()))
                .exists()
            {
                assert_fixture_command(&fixture, kind);
                exercised.push(format!(
                    "{}:{}",
                    fixture.file_name().unwrap().to_string_lossy(),
                    kind.name()
                ));
            }
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

#[derive(Debug, Clone, Copy)]
enum CommandKind {
    Check,
    GraphJson,
    Query,
}

impl CommandKind {
    const ALL: [Self; 3] = [Self::Check, Self::GraphJson, Self::Query];

    fn name(self) -> &'static str {
        match self {
            Self::Check => "check",
            Self::GraphJson => "graph",
            Self::Query => "query",
        }
    }
}

fn assert_fixture_command(fixture: &Path, kind: CommandKind) {
    let repo = fixture.join("repo");
    let expected_exit = read_expected_exit(&fixture, kind);
    let expected_stdout = read_optional(&fixture, &format!("expected.{}.stdout", kind.name()))
        .or_else(|| read_optional(&fixture, &format!("expected.{}.stdout.json", kind.name())))
        .unwrap_or_default();
    let expected_stderr =
        read_optional(&fixture, &format!("expected.{}.stderr", kind.name())).unwrap_or_default();

    let output = match kind {
        CommandKind::Check => ochams().arg("check").arg(&repo).output().expect("check"),
        CommandKind::GraphJson => ochams()
            .arg("graph")
            .arg(&repo)
            .arg("--format")
            .arg("json")
            .output()
            .expect("graph"),
        CommandKind::Query => ochams()
            .arg("query")
            .arg(&repo)
            .arg(read_required(&fixture, "query.symbol").trim())
            .output()
            .expect("query"),
    };

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

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tests")
        .join("fixtures")
}

fn fixture_paths() -> Vec<PathBuf> {
    let mut fixtures = fs::read_dir(fixtures_root())
        .expect("fixtures root")
        .map(|entry| entry.expect("fixture entry").path())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    fixtures.sort();
    fixtures
}

fn read_expected_exit(fixture: &Path, kind: CommandKind) -> i32 {
    read_required(fixture, &format!("expected.{}.exit", kind.name()))
        .trim()
        .parse()
        .expect("expected exit code")
}

fn read_required(fixture: &Path, rel_path: &str) -> String {
    fs::read_to_string(fixture.join(rel_path)).unwrap_or_else(|error| {
        panic!(
            "could not read fixture file {}: {error}",
            fixture.join(rel_path).display()
        )
    })
}

fn read_optional(fixture: &Path, rel_path: &str) -> Option<String> {
    match fs::read_to_string(fixture.join(rel_path)) {
        Ok(content) => Some(content),
        Err(error) if error.kind() == ErrorKind::NotFound => None,
        Err(error) => panic!(
            "could not read fixture file {}: {error}",
            fixture.join(rel_path).display()
        ),
    }
}
