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
            vec!["scan", ".", "--code", ".", "--format", "yaml"],
            "OCH020 unsupported scan format `yaml`\n",
        ),
        (vec!["graph", "."], usage()),
        (vec!["nope", "."], usage()),
        (vec!["check", ".", "extra"], usage()),
        (vec!["query", "."], usage()),
        (vec!["scan", ".", "--code", "."], usage()),
    ];

    for (args, expected_stderr) in cases {
        let output = ochams().args(args).output().expect("command");
        assert_output(output, 1, "", expected_stderr);
    }
}

#[test]
fn scan_absolute_code_root_under_relative_root_renders_root_relative() {
    let fixture = fixtures_root().join("scan-anchors");
    let repo = fixture.join("repo");
    let output = ochams()
        .current_dir(&repo)
        .arg("scan")
        .arg(".")
        .arg("--code")
        .arg(repo.join("src"))
        .arg("--format")
        .arg("json")
        .output()
        .expect("scan");

    assert_eq!(output.status.code().unwrap_or(1), 0);
    let stdout = String::from_utf8(output.stdout).expect("stdout");
    assert!(stdout.contains("\"codeRoot\": \"src\""));
}

#[test]
fn scan_normalizes_relative_code_root_before_architecture_exclusion() {
    let fixture = fixtures_root().join("scan-architecture-skipped");
    let repo = fixture.join("repo");
    fs::create_dir_all(repo.join("src")).expect("src");

    let output = ochams()
        .arg("scan")
        .arg(&repo)
        .arg("--code")
        .arg("src/../architecture")
        .arg("--format")
        .arg("json")
        .output()
        .expect("scan");

    assert_output(
        output,
        0,
        &read_required(&fixture, "expected.scan.stdout.json").replace(
            "\"codeRoot\": \"architecture\"",
            "\"codeRoot\": \"src/../architecture\"",
        ),
        "",
    );
}

#[cfg(unix)]
#[test]
fn scan_normalizes_symlinked_code_root_before_architecture_exclusion() {
    use std::os::unix::fs::symlink;

    let source_fixture = fixtures_root().join("scan-architecture-skipped");
    let temp = temp_repo("ochams-symlink-scan");
    copy_dir(&source_fixture.join("repo"), &temp).expect("copy fixture");
    symlink(temp.join("architecture"), temp.join("src-link")).expect("symlink");

    let output = ochams()
        .arg("scan")
        .arg(&temp)
        .arg("--code")
        .arg("src-link")
        .arg("--format")
        .arg("json")
        .output()
        .expect("scan");

    assert_output(
        output,
        0,
        &read_required(&source_fixture, "expected.scan.stdout.json").replace(
            "\"codeRoot\": \"architecture\"",
            "\"codeRoot\": \"src-link\"",
        ),
        "",
    );
}

#[test]
fn scan_includes_hidden_code_paths() {
    let source_fixture = fixtures_root().join("scan-anchors");
    let temp = temp_repo("ochams-hidden-scan");
    copy_dir(&source_fixture.join("repo"), &temp).expect("copy fixture");
    fs::create_dir_all(temp.join("src/.generated")).expect("hidden dir");
    fs::rename(
        temp.join("src/scheduling.rs"),
        temp.join("src/.generated/anchor.rs"),
    )
    .expect("move anchor");

    let output = ochams()
        .arg("scan")
        .arg(&temp)
        .arg("--code")
        .arg("src")
        .arg("--format")
        .arg("json")
        .output()
        .expect("scan");

    assert_eq!(output.status.code().unwrap_or(1), 0);
    let stdout = String::from_utf8(output.stdout).expect("stdout");
    assert!(stdout.contains("\"path\": \".generated/anchor.rs\""));
    assert!(stdout.contains("VetClinic.Capabilities.Commands.ScheduleAppointment"));
}

#[test]
fn scan_honors_code_root_gitignore_without_git_repository() {
    let source_fixture = fixtures_root().join("scan-anchors");
    let temp = temp_repo("ochams-ignore-scan");
    copy_dir(&source_fixture.join("repo"), &temp).expect("copy fixture");
    fs::write(temp.join("src/.gitignore"), "scheduling.rs\n").expect("gitignore");

    let output = ochams()
        .arg("scan")
        .arg(&temp)
        .arg("--code")
        .arg("src")
        .arg("--format")
        .arg("json")
        .output()
        .expect("scan");

    assert_eq!(output.status.code().unwrap_or(1), 0);
    let stdout = String::from_utf8(output.stdout).expect("stdout");
    assert!(stdout.contains("\"sourceAnchors\": []"));
    assert!(stdout.contains("\"observedEdges\": []"));
}

#[test]
fn scan_honors_code_root_search_ignore_files_without_git_repository() {
    let source_fixture = fixtures_root().join("scan-anchors");
    for (ignore_file, prefix) in [
        (".rgignore", "ochams-rgignore-scan"),
        (".fdignore", "ochams-fdignore-scan"),
    ] {
        let temp = temp_repo(prefix);
        copy_dir(&source_fixture.join("repo"), &temp).expect("copy fixture");
        fs::write(temp.join("src").join(ignore_file), "scheduling.rs\n").expect(ignore_file);

        let output = ochams()
            .arg("scan")
            .arg(&temp)
            .arg("--code")
            .arg("src")
            .arg("--format")
            .arg("json")
            .output()
            .expect("scan");

        assert_eq!(output.status.code().unwrap_or(1), 0);
        let stdout = String::from_utf8(output.stdout).expect("stdout");
        assert!(stdout.contains("\"sourceAnchors\": []"));
        assert!(stdout.contains("\"observedEdges\": []"));
    }
}

#[test]
fn scan_excludes_vcs_metadata_while_including_other_hidden_paths() {
    let source_fixture = fixtures_root().join("scan-anchors");
    let temp = temp_repo("ochams-vcs-metadata-scan");
    copy_dir(&source_fixture.join("repo"), &temp).expect("copy fixture");
    fs::create_dir_all(temp.join(".git")).expect("git dir");
    fs::write(
        temp.join(".git/HEAD"),
        "@realizes VetClinic.Capabilities.Commands.ScheduleAppointment\n",
    )
    .expect("git head");
    fs::create_dir_all(temp.join(".generated")).expect("generated dir");
    fs::write(
        temp.join(".generated/anchor.rs"),
        "@realizes VetClinic.Capabilities.Commands.ScheduleAppointment\n",
    )
    .expect("hidden anchor");

    let output = ochams()
        .arg("scan")
        .arg(&temp)
        .arg("--code")
        .arg(".")
        .arg("--format")
        .arg("json")
        .output()
        .expect("scan");

    assert_eq!(output.status.code().unwrap_or(1), 0);
    let stdout = String::from_utf8(output.stdout).expect("stdout");
    assert!(stdout.contains("\"path\": \".generated/anchor.rs\""));
    assert!(!stdout.contains(".git/HEAD"));
}

#[test]
fn scan_empty_when_code_root_is_vcs_metadata() {
    let source_fixture = fixtures_root().join("scan-anchors");
    let temp = temp_repo("ochams-vcs-root-scan");
    copy_dir(&source_fixture.join("repo"), &temp).expect("copy fixture");
    fs::create_dir_all(temp.join(".git")).expect("git dir");
    fs::write(
        temp.join(".git/HEAD"),
        "@realizes VetClinic.Capabilities.Commands.ScheduleAppointment\n",
    )
    .expect("git head");

    let output = ochams()
        .arg("scan")
        .arg(&temp)
        .arg("--code")
        .arg(".git")
        .arg("--format")
        .arg("json")
        .output()
        .expect("scan");

    assert_eq!(output.status.code().unwrap_or(1), 0);
    let stdout = String::from_utf8(output.stdout).expect("stdout");
    assert!(stdout.contains("\"codeRoot\": \".git\""));
    assert!(stdout.contains("\"sourceAnchors\": []"));
    assert!(stdout.contains("\"observedEdges\": []"));
    assert!(!stdout.contains("ScheduleAppointment"));
}

#[test]
fn scan_empty_when_code_root_is_vcs_metadata_file() {
    let source_fixture = fixtures_root().join("scan-anchors");
    let temp = temp_repo("ochams-vcs-file-scan");
    copy_dir(&source_fixture.join("repo"), &temp).expect("copy fixture");
    fs::create_dir_all(temp.join(".git")).expect("git dir");
    fs::write(
        temp.join(".git/HEAD"),
        "@realizes VetClinic.Capabilities.Commands.ScheduleAppointment\n",
    )
    .expect("git head");

    let output = ochams()
        .arg("scan")
        .arg(&temp)
        .arg("--code")
        .arg(".git/HEAD")
        .arg("--format")
        .arg("json")
        .output()
        .expect("scan");

    assert_eq!(output.status.code().unwrap_or(1), 0);
    let stdout = String::from_utf8(output.stdout).expect("stdout");
    assert!(stdout.contains("\"codeRoot\": \".git/HEAD\""));
    assert!(stdout.contains("\"sourceAnchors\": []"));
    assert!(stdout.contains("\"observedEdges\": []"));
    assert!(!stdout.contains("ScheduleAppointment"));
}

#[test]
fn scan_empty_when_code_root_is_nested_vcs_metadata_file() {
    let source_fixture = fixtures_root().join("scan-anchors");
    let temp = temp_repo("ochams-nested-vcs-file-scan");
    copy_dir(&source_fixture.join("repo"), &temp).expect("copy fixture");
    fs::create_dir_all(temp.join("src/.git")).expect("git dir");
    fs::write(
        temp.join("src/.git/HEAD"),
        "@realizes VetClinic.Capabilities.Commands.ScheduleAppointment\n",
    )
    .expect("git head");

    let output = ochams()
        .arg("scan")
        .arg(&temp)
        .arg("--code")
        .arg("src/.git/HEAD")
        .arg("--format")
        .arg("json")
        .output()
        .expect("scan");

    assert_eq!(output.status.code().unwrap_or(1), 0);
    let stdout = String::from_utf8(output.stdout).expect("stdout");
    assert!(stdout.contains("\"codeRoot\": \"src/.git/HEAD\""));
    assert!(stdout.contains("\"sourceAnchors\": []"));
    assert!(stdout.contains("\"observedEdges\": []"));
    assert!(!stdout.contains("ScheduleAppointment"));
}

#[test]
fn scan_empty_when_absolute_external_code_root_is_vcs_metadata() {
    let source_fixture = fixtures_root().join("scan-anchors");
    let root = source_fixture.join("repo");
    let external = temp_repo("ochams-external-vcs-root-scan");
    fs::create_dir_all(external.join(".git")).expect("git dir");
    fs::write(
        external.join(".git/HEAD"),
        "@realizes VetClinic.Capabilities.Commands.ScheduleAppointment\n",
    )
    .expect("git head");

    let output = ochams()
        .arg("scan")
        .arg(&root)
        .arg("--code")
        .arg(external.join(".git"))
        .arg("--format")
        .arg("json")
        .output()
        .expect("scan");

    assert_eq!(output.status.code().unwrap_or(1), 0);
    let stdout = String::from_utf8(output.stdout).expect("stdout");
    assert!(stdout.contains("\"sourceAnchors\": []"));
    assert!(stdout.contains("\"observedEdges\": []"));
    assert!(!stdout.contains("ScheduleAppointment"));
}

#[test]
fn scan_empty_when_absolute_external_code_root_is_vcs_metadata_file() {
    let source_fixture = fixtures_root().join("scan-anchors");
    let root = source_fixture.join("repo");
    let external = temp_repo("ochams-external-vcs-file-scan");
    fs::create_dir_all(external.join(".git/objects/ab")).expect("git object dir");
    fs::write(
        external.join(".git/objects/ab/cd"),
        "@realizes VetClinic.Capabilities.Commands.ScheduleAppointment\n",
    )
    .expect("git object");

    let output = ochams()
        .arg("scan")
        .arg(&root)
        .arg("--code")
        .arg(external.join(".git/objects/ab/cd"))
        .arg("--format")
        .arg("json")
        .output()
        .expect("scan");

    assert_eq!(output.status.code().unwrap_or(1), 0);
    let stdout = String::from_utf8(output.stdout).expect("stdout");
    assert!(stdout.contains("\"sourceAnchors\": []"));
    assert!(stdout.contains("\"observedEdges\": []"));
    assert!(!stdout.contains("ScheduleAppointment"));
}

#[test]
fn scan_excludes_vcs_control_files() {
    let source_fixture = fixtures_root().join("scan-anchors");
    let temp = temp_repo("ochams-vcs-control-file-scan");
    copy_dir(&source_fixture.join("repo"), &temp).expect("copy fixture");
    let anchor = "# @realizes VetClinic.Capabilities.Commands.ScheduleAppointment\n";
    for control_file in [
        ".gitignore",
        ".gitattributes",
        ".gitmodules",
        ".git-blame-ignore-revs",
        ".hgignore",
        ".ignore",
        ".rgignore",
        ".fdignore",
    ] {
        fs::write(temp.join(control_file), anchor).expect(control_file);
    }
    fs::create_dir_all(temp.join(".jj")).expect("jj dir");
    fs::write(temp.join(".jj/repo"), anchor).expect("jj metadata");
    fs::create_dir_all(temp.join(".github/workflows")).expect("github workflow dir");
    fs::write(
        temp.join(".github/workflows/architecture.yml"),
        "@realizes VetClinic.Capabilities.Commands.ScheduleAppointment\n",
    )
    .expect("workflow");

    let output = ochams()
        .arg("scan")
        .arg(&temp)
        .arg("--code")
        .arg(".")
        .arg("--format")
        .arg("json")
        .output()
        .expect("scan");

    assert_eq!(output.status.code().unwrap_or(1), 0);
    let stdout = String::from_utf8(output.stdout).expect("stdout");
    for control_file in [
        ".gitignore",
        ".gitattributes",
        ".gitmodules",
        ".git-blame-ignore-revs",
        ".hgignore",
        ".ignore",
        ".rgignore",
        ".fdignore",
        ".jj/repo",
    ] {
        assert!(!stdout.contains(&format!("\"path\": \"{control_file}\"")));
    }
    assert!(stdout.contains("\"path\": \".github/workflows/architecture.yml\""));
}

#[test]
fn scan_does_not_treat_vcs_named_ancestors_as_metadata() {
    let source_fixture = fixtures_root().join("scan-anchors");
    let temp_parent = temp_repo("ochams-vcs-ancestor-scan").join(".git");
    let temp = temp_parent.join("repo");
    copy_dir(&source_fixture.join("repo"), &temp).expect("copy fixture");

    let output = ochams()
        .arg("scan")
        .arg(&temp)
        .arg("--code")
        .arg("src")
        .arg("--format")
        .arg("json")
        .output()
        .expect("scan");

    assert_eq!(output.status.code().unwrap_or(1), 0);
    let stdout = String::from_utf8(output.stdout).expect("stdout");
    assert!(stdout.contains("\"path\": \"scheduling.rs\""));
    assert!(stdout.contains("VetClinic.Capabilities.Commands.ScheduleAppointment"));
}

#[test]
fn scan_absolute_external_code_root_under_vcs_named_parent_is_not_suppressed() {
    let source_fixture = fixtures_root().join("scan-anchors");
    let root = source_fixture.join("repo");
    let external_parent = temp_repo("ochams-external-vcs-ancestor-scan").join(".git");
    let external_src = external_parent.join("repo/src");
    fs::create_dir_all(&external_src).expect("external src");
    fs::copy(
        source_fixture.join("repo/src/scheduling.rs"),
        external_src.join("scheduling.rs"),
    )
    .expect("copy source");

    let output = ochams()
        .arg("scan")
        .arg(&root)
        .arg("--code")
        .arg(&external_src)
        .arg("--format")
        .arg("json")
        .output()
        .expect("scan");

    assert_eq!(output.status.code().unwrap_or(1), 0);
    let stdout = String::from_utf8(output.stdout).expect("stdout");
    assert!(stdout.contains("\"path\": \"scheduling.rs\""));
    assert!(stdout.contains("VetClinic.Capabilities.Commands.ScheduleAppointment"));
}

#[derive(Debug, Clone, Copy)]
enum CommandKind {
    Check,
    GraphJson,
    Query,
    Scan,
}

impl CommandKind {
    const ALL: [Self; 4] = [Self::Check, Self::GraphJson, Self::Query, Self::Scan];

    fn name(self) -> &'static str {
        match self {
            Self::Check => "check",
            Self::GraphJson => "graph",
            Self::Query => "query",
            Self::Scan => "scan",
        }
    }
}

fn assert_fixture_command(fixture: &Path, kind: CommandKind) {
    let repo = fixture.join("repo");
    let expected_exit = read_expected_exit(fixture, kind);
    let expected_stdout = read_optional(fixture, &format!("expected.{}.stdout", kind.name()))
        .or_else(|| read_optional(fixture, &format!("expected.{}.stdout.json", kind.name())))
        .unwrap_or_default();
    let expected_stderr =
        read_optional(fixture, &format!("expected.{}.stderr", kind.name())).unwrap_or_default();

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
            .arg(read_required(fixture, "query.symbol").trim())
            .output()
            .expect("query"),
        CommandKind::Scan => {
            let code_root = read_required(fixture, "scan.code");
            ochams()
                .arg("scan")
                .arg(&repo)
                .arg("--code")
                .arg(code_root.trim())
                .arg("--format")
                .arg("json")
                .output()
                .expect("scan")
        }
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

fn usage() -> &'static str {
    "OCH020 usage: ochams check <root> | ochams graph <root> --format json | ochams query <root> <symbol> | ochams scan <root> --code <path> --format json\n"
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

fn copy_dir(from: &Path, to: &Path) -> std::io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let from_path = entry.path();
        let to_path = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&from_path, &to_path)?;
        } else {
            fs::copy(&from_path, &to_path)?;
        }
    }
    Ok(())
}

fn temp_repo(prefix: &str) -> PathBuf {
    use std::time::{SystemTime, UNIX_EPOCH};

    std::env::temp_dir().join(format!(
        "{}-{}",
        prefix,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos()
    ))
}
