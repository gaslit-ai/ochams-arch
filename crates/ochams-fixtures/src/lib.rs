use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use tempfile::{Builder, TempDir};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommandKind {
    Check,
    GraphJson,
    Query,
}

#[derive(Debug, Clone, Copy)]
struct CommandSpec {
    name: &'static str,
    expected_exit_path: &'static str,
    expected_stderr_path: &'static str,
    allowed_stdout_path: &'static str,
    stale_stdout_paths: &'static [&'static str],
    stdout_cleanup_paths: &'static [&'static str],
    query_symbol_path: Option<&'static str>,
}

const CHECK_STDOUT_PATHS: &[&str] = &["expected.check.stdout", "expected.check.stdout.json"];
const GRAPH_STDOUT_PATHS: &[&str] = &["expected.graph.stdout", "expected.graph.stdout.json"];
const QUERY_STDOUT_PATHS: &[&str] = &["expected.query.stdout", "expected.query.stdout.json"];

impl CommandKind {
    pub const ALL: [Self; 3] = [Self::Check, Self::GraphJson, Self::Query];

    fn spec(self) -> CommandSpec {
        match self {
            Self::Check => CommandSpec {
                name: "check",
                expected_exit_path: "expected.check.exit",
                expected_stderr_path: "expected.check.stderr",
                allowed_stdout_path: "expected.check.stdout",
                stale_stdout_paths: &["expected.check.stdout.json"],
                stdout_cleanup_paths: CHECK_STDOUT_PATHS,
                query_symbol_path: None,
            },
            Self::GraphJson => CommandSpec {
                name: "graph",
                expected_exit_path: "expected.graph.exit",
                expected_stderr_path: "expected.graph.stderr",
                allowed_stdout_path: "expected.graph.stdout.json",
                stale_stdout_paths: &["expected.graph.stdout"],
                stdout_cleanup_paths: GRAPH_STDOUT_PATHS,
                query_symbol_path: None,
            },
            Self::Query => CommandSpec {
                name: "query",
                expected_exit_path: "expected.query.exit",
                expected_stderr_path: "expected.query.stderr",
                allowed_stdout_path: "expected.query.stdout",
                stale_stdout_paths: &["expected.query.stdout.json"],
                stdout_cleanup_paths: QUERY_STDOUT_PATHS,
                query_symbol_path: Some("query.symbol"),
            },
        }
    }

    pub fn name(self) -> &'static str {
        self.spec().name
    }

    fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.name() == value)
    }

    pub fn expected_stdout_write_path(self, stdout: &[u8]) -> Option<&'static str> {
        if stdout.is_empty() {
            return None;
        }

        Some(self.allowed_stdout_path())
    }

    pub fn stale_stdout_paths(self) -> &'static [&'static str] {
        self.spec().stale_stdout_paths
    }

    pub fn stdout_cleanup_paths(self) -> &'static [&'static str] {
        self.spec().stdout_cleanup_paths
    }

    pub fn expected_exit_path(self) -> &'static str {
        self.spec().expected_exit_path
    }

    pub fn expected_stderr_path(self) -> &'static str {
        self.spec().expected_stderr_path
    }

    pub fn allowed_stdout_path(self) -> &'static str {
        self.spec().allowed_stdout_path
    }

    pub fn query_symbol_path(self) -> Option<&'static str> {
        self.spec().query_symbol_path
    }

    fn allowed_expected_paths(self) -> [&'static str; 3] {
        [
            self.expected_exit_path(),
            self.expected_stderr_path(),
            self.allowed_stdout_path(),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct Fixture {
    pub path: PathBuf,
    pub commands: Vec<CommandKind>,
}

#[derive(Debug)]
pub struct RepoDir {
    temp_dir: TempDir,
}

impl RepoDir {
    pub fn path(&self) -> &Path {
        self.temp_dir.path()
    }
}

impl AsRef<Path> for RepoDir {
    fn as_ref(&self) -> &Path {
        self.path()
    }
}

pub fn find_workspace_root(start: &Path) -> Result<PathBuf, String> {
    start
        .ancestors()
        .find(|path| {
            path.join("xtask").is_dir()
                && fixture_root(path).is_dir()
                && seed_root(path).is_dir()
                && path.join("Cargo.toml").is_file()
        })
        .map(Path::to_path_buf)
        .ok_or_else(|| format!("could not find workspace root from {}", start.display()))
}

pub fn fixture_root(workspace: &Path) -> PathBuf {
    workspace.join("tests").join("fixtures")
}

pub fn seed_root(workspace: &Path) -> PathBuf {
    workspace.join("tests").join("seeds")
}

pub fn seed_case_path(workspace: &Path, case: &str) -> PathBuf {
    seed_root(workspace).join(case)
}

pub fn temp_repo(prefix: &str) -> Result<RepoDir, String> {
    Builder::new()
        .prefix(prefix)
        .tempdir()
        .map(|temp_dir| RepoDir { temp_dir })
        .map_err(|error| format!("could not create temp repo `{prefix}`: {error}"))
}

pub fn materialize_seed(workspace: &Path, case: &str, prefix: &str) -> Result<RepoDir, String> {
    let root = temp_repo(prefix)?;
    copy_dir_all(&seed_case_path(workspace, case), root.path())?;
    Ok(root)
}

pub fn fixture_paths(workspace: &Path) -> Result<Vec<Fixture>, String> {
    let root = fixture_root(workspace);
    let mut paths = fs::read_dir(&root)
        .map_err(|error| format!("could not read fixture root {}: {error}", root.display()))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("could not read fixture entry: {error}"))?;
    paths.retain(|path| path.is_dir());
    paths.sort();

    paths
        .into_iter()
        .map(|path| {
            let commands = read_commands(&path)?;
            validate_fixture_contract(&path, &commands)?;
            Ok(Fixture { path, commands })
        })
        .collect()
}

pub fn read_commands(fixture: &Path) -> Result<Vec<CommandKind>, String> {
    let manifest = read_required(fixture, "commands.txt")?;
    let mut commands = Vec::new();
    let mut seen = BTreeSet::new();

    for (line_index, line) in manifest.lines().enumerate() {
        let command = line.trim();
        if command.is_empty() || command.starts_with('#') {
            continue;
        }

        let Some(kind) = CommandKind::parse(command) else {
            return Err(format!(
                "{} commands.txt:{} unknown command `{command}`",
                display_fixture(fixture),
                line_index + 1
            ));
        };

        if !seen.insert(kind) {
            return Err(format!(
                "{} commands.txt:{} duplicate command `{command}`",
                display_fixture(fixture),
                line_index + 1
            ));
        }
        commands.push(kind);
    }

    if commands.is_empty() {
        return Err(format!(
            "{} commands.txt declares no commands",
            display_fixture(fixture)
        ));
    }

    Ok(commands)
}

pub fn read_expected_exit(fixture: &Path, kind: CommandKind) -> Result<i32, String> {
    read_required(fixture, kind.expected_exit_path())?
        .trim()
        .parse()
        .map_err(|error| {
            format!(
                "could not parse {} expected {} exit: {error}",
                display_fixture(fixture),
                kind.name()
            )
        })
}

pub fn expected_stdout(fixture: &Path, kind: CommandKind) -> Result<String, String> {
    for stale in kind.stale_stdout_paths() {
        if fixture.join(stale).exists() {
            return Err(format!(
                "{} has stale or misnamed stdout fixture {stale}",
                display_fixture(fixture)
            ));
        }
    }

    read_optional(fixture, kind.allowed_stdout_path()).map(|content| content.unwrap_or_default())
}

pub fn expected_stderr(fixture: &Path, kind: CommandKind) -> Result<String, String> {
    read_optional(fixture, kind.expected_stderr_path()).map(|content| content.unwrap_or_default())
}

pub fn command_args(
    fixture: &Path,
    kind: CommandKind,
    repo: &Path,
) -> Result<Vec<OsString>, String> {
    let mut args = vec![OsString::from(kind.name()), repo.as_os_str().to_owned()];

    match kind {
        CommandKind::Check => {}
        CommandKind::GraphJson => {
            args.push(OsString::from("--format"));
            args.push(OsString::from("json"));
        }
        CommandKind::Query => {
            let symbol = read_required(
                fixture,
                kind.query_symbol_path()
                    .expect("query command requires query symbol path"),
            )?;
            args.push(OsString::from(symbol.trim()));
        }
    }

    Ok(args)
}

pub fn read_required(fixture: &Path, rel_path: &str) -> Result<String, String> {
    read_text_file(fixture, rel_path, true).map(Option::unwrap)
}

pub fn read_optional(fixture: &Path, rel_path: &str) -> Result<Option<String>, String> {
    read_text_file(fixture, rel_path, false)
}

pub fn display_fixture(fixture: &Path) -> String {
    fixture
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("<unknown>")
        .to_owned()
}

pub fn copy_dir_all(source: &Path, destination: &Path) -> Result<(), String> {
    let entries = fs::read_dir(source).map_err(|error| {
        format!(
            "could not read source directory {}: {error}",
            source.display()
        )
    })?;
    fs::create_dir_all(destination).map_err(|error| {
        format!(
            "could not create directory {}: {error}",
            destination.display()
        )
    })?;

    for entry in entries {
        let entry = entry.map_err(|error| {
            format!(
                "could not read directory entry in {}: {error}",
                source.display()
            )
        })?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry.file_type().map_err(|error| {
            format!(
                "could not read entry type {}: {error}",
                source_path.display()
            )
        })?;

        if file_type.is_dir() {
            copy_dir_all(&source_path, &destination_path)?;
        } else {
            fs::copy(&source_path, &destination_path).map_err(|error| {
                format!(
                    "could not copy {} to {}: {error}",
                    source_path.display(),
                    destination_path.display()
                )
            })?;
        }
    }

    Ok(())
}

fn read_text_file(
    fixture: &Path,
    rel_path: &str,
    required: bool,
) -> Result<Option<String>, String> {
    match fs::read_to_string(fixture.join(rel_path)) {
        Ok(content) => Ok(Some(content)),
        Err(error) if !required && error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!(
            "could not read fixture file {}: {error}",
            fixture.join(rel_path).display()
        )),
    }
}

fn validate_fixture_contract(fixture: &Path, commands: &[CommandKind]) -> Result<(), String> {
    let declared = commands.iter().copied().collect::<BTreeSet<_>>();
    let query_symbol_path = CommandKind::Query
        .query_symbol_path()
        .expect("query command path is defined");
    if declared.contains(&CommandKind::Query) {
        read_required(fixture, query_symbol_path)?;
    } else if fixture.join(query_symbol_path).exists() {
        return Err(format!(
            "{} has query.symbol but commands.txt does not declare `query`",
            display_fixture(fixture)
        ));
    }

    for kind in commands {
        read_expected_exit(fixture, *kind)?;
    }

    for entry in fs::read_dir(fixture)
        .map_err(|error| format!("could not read fixture {}: {error}", fixture.display()))?
    {
        let entry = entry.map_err(|error| {
            format!(
                "could not read fixture entry in {}: {error}",
                fixture.display()
            )
        })?;
        if !entry
            .file_type()
            .map_err(|error| {
                format!(
                    "could not read fixture entry type {}: {error}",
                    entry.path().display()
                )
            })?
            .is_file()
        {
            continue;
        }

        let file_name = entry.file_name().into_string().map_err(|_| {
            format!(
                "{} has a non-UTF-8 fixture file name",
                display_fixture(fixture)
            )
        })?;
        let Some(rest) = file_name.strip_prefix("expected.") else {
            continue;
        };
        let Some(command_name) = rest.split('.').next() else {
            return Err(format!(
                "{} has malformed expected fixture file {file_name}",
                display_fixture(fixture)
            ));
        };
        let Some(kind) = CommandKind::parse(command_name) else {
            return Err(format!(
                "{} has expected fixture file for unknown command `{command_name}`",
                display_fixture(fixture)
            ));
        };

        if !declared.contains(&kind) {
            return Err(format!(
                "{} has expected fixture file {file_name} but commands.txt does not declare `{}`",
                display_fixture(fixture),
                kind.name()
            ));
        }

        if !kind
            .allowed_expected_paths()
            .iter()
            .any(|allowed| allowed == &file_name)
        {
            return Err(format!(
                "{} has unexpected expected fixture file {file_name}",
                display_fixture(fixture)
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

    #[test]
    fn command_manifest_preserves_explicit_order() {
        let fixture = temp_fixture("command_manifest_preserves_explicit_order");
        write(
            &fixture,
            "commands.txt",
            "# public commands\nquery\ncheck\ngraph\n",
        );

        assert_eq!(
            read_commands(&fixture).expect("commands"),
            vec![
                CommandKind::Query,
                CommandKind::Check,
                CommandKind::GraphJson
            ]
        );
    }

    #[test]
    fn command_manifest_rejects_unknown_duplicate_and_empty_manifests() {
        let unknown = temp_fixture("command_manifest_rejects_unknown");
        write(&unknown, "commands.txt", "check\nscan\n");
        assert!(
            read_commands(&unknown)
                .expect_err("unknown command")
                .contains("unknown command `scan`")
        );

        let duplicate = temp_fixture("command_manifest_rejects_duplicate");
        write(&duplicate, "commands.txt", "graph\ngraph\n");
        assert!(
            read_commands(&duplicate)
                .expect_err("duplicate command")
                .contains("duplicate command `graph`")
        );

        let empty = temp_fixture("command_manifest_rejects_empty");
        write(&empty, "commands.txt", "\n# no commands\n");
        assert!(
            read_commands(&empty)
                .expect_err("empty command manifest")
                .contains("declares no commands")
        );
    }

    #[test]
    fn fixture_discovery_uses_manifest_authority() {
        let workspace = temp_workspace("fixture_discovery_uses_manifest_authority");
        let alpha = fixture_root(&workspace).join("alpha");
        let beta = fixture_root(&workspace).join("beta");
        write(&beta, "commands.txt", "query\n");
        write(&beta, "query.symbol", "VetClinic.Domain.Resources.Pet\n");
        write(&beta, "expected.query.exit", "0\n");
        write(&alpha, "commands.txt", "check\ngraph\n");
        write(&alpha, "expected.check.exit", "0\n");
        write(&alpha, "expected.graph.exit", "0\n");
        write(&fixture_root(&workspace), "README.md", "# fixtures\n");

        let fixtures = fixture_paths(&workspace).expect("fixtures");

        assert_eq!(
            fixtures
                .iter()
                .map(|fixture| display_fixture(&fixture.path))
                .collect::<Vec<_>>(),
            vec!["alpha", "beta"]
        );
        assert_eq!(
            fixtures
                .iter()
                .map(|fixture| fixture.commands.clone())
                .collect::<Vec<_>>(),
            vec![
                vec![CommandKind::Check, CommandKind::GraphJson],
                vec![CommandKind::Query]
            ]
        );
    }

    #[test]
    fn expected_exit_is_required_and_numeric() {
        let fixture = temp_fixture("expected_exit_is_required_and_numeric");
        write(&fixture, "expected.check.exit", "0\n");

        assert_eq!(
            read_expected_exit(&fixture, CommandKind::Check).expect("exit"),
            0
        );

        write(&fixture, "expected.check.exit", "not-a-number\n");
        assert!(
            read_expected_exit(&fixture, CommandKind::Check)
                .expect_err("invalid exit")
                .contains("could not parse")
        );

        let missing = temp_fixture("expected_exit_is_required");
        assert!(
            read_expected_exit(&missing, CommandKind::Check)
                .expect_err("missing exit")
                .contains("could not read fixture file")
        );
    }

    #[test]
    fn fixture_discovery_rejects_orphaned_expected_files_and_query_symbol() {
        let workspace = temp_workspace("fixture_discovery_rejects_orphans");
        let orphaned_expected = fixture_root(&workspace).join("orphaned-expected");
        write(&orphaned_expected, "commands.txt", "check\n");
        write(&orphaned_expected, "expected.check.exit", "0\n");
        write(&orphaned_expected, "expected.graph.exit", "0\n");
        assert!(
            fixture_paths(&workspace)
                .expect_err("orphaned expected file")
                .contains("does not declare `graph`")
        );

        let workspace = temp_workspace("fixture_discovery_rejects_query_symbol");
        let orphaned_query = fixture_root(&workspace).join("orphaned-query");
        write(&orphaned_query, "commands.txt", "check\n");
        write(&orphaned_query, "expected.check.exit", "0\n");
        write(
            &orphaned_query,
            "query.symbol",
            "VetClinic.Domain.Resources.Pet\n",
        );
        assert!(
            fixture_paths(&workspace)
                .expect_err("orphaned query symbol")
                .contains("does not declare `query`")
        );
    }

    #[test]
    fn fixture_discovery_rejects_unexpected_expected_file_names() {
        let workspace = temp_workspace("fixture_discovery_rejects_unexpected_expected_files");
        let fixture = fixture_root(&workspace).join("unexpected");
        write(&fixture, "commands.txt", "graph\n");
        write(&fixture, "expected.graph.exit", "0\n");
        write(&fixture, "expected.graph.stdout", "{}\n");

        assert!(
            fixture_paths(&workspace)
                .expect_err("unexpected expected file")
                .contains("unexpected expected fixture file expected.graph.stdout")
        );
    }

    #[test]
    fn expected_streams_default_to_empty_when_absent() {
        let fixture = temp_fixture("expected_streams_default_to_empty_when_absent");

        assert_eq!(
            expected_stdout(&fixture, CommandKind::Check).expect("stdout"),
            ""
        );
        assert_eq!(
            expected_stderr(&fixture, CommandKind::Check).expect("stderr"),
            ""
        );
    }

    #[test]
    fn expected_streams_reject_malformed_utf8() {
        let fixture = temp_fixture("expected_streams_reject_malformed_utf8");
        write_bytes(&fixture, "expected.check.stderr", &[0xff]);

        assert!(
            expected_stderr(&fixture, CommandKind::Check)
                .expect_err("non-UTF-8 stderr")
                .contains("could not read fixture file")
        );
    }

    #[test]
    fn stdout_contract_is_command_specific() {
        let graph = temp_fixture("stdout_contract_graph");
        write(&graph, "expected.graph.stdout.json", "{\n}\n");
        assert_eq!(
            expected_stdout(&graph, CommandKind::GraphJson).expect("graph stdout"),
            "{\n}\n"
        );

        write(&graph, "expected.graph.stdout", "stale\n");
        assert!(
            expected_stdout(&graph, CommandKind::GraphJson)
                .expect_err("stale graph stdout")
                .contains("stale or misnamed stdout fixture expected.graph.stdout")
        );

        let query = temp_fixture("stdout_contract_query");
        write(&query, "expected.query.stdout", "symbol: x\n");
        assert_eq!(
            expected_stdout(&query, CommandKind::Query).expect("query stdout"),
            "symbol: x\n"
        );

        write(&query, "expected.query.stdout.json", "{}\n");
        assert!(
            expected_stdout(&query, CommandKind::Query)
                .expect_err("stale query stdout")
                .contains("stale or misnamed stdout fixture expected.query.stdout.json")
        );
    }

    #[test]
    fn regeneration_paths_remove_empty_stdout_authority() {
        assert_eq!(
            CommandKind::GraphJson.expected_stdout_write_path(b"{}"),
            Some("expected.graph.stdout.json")
        );
        assert_eq!(
            CommandKind::Query.expected_stdout_write_path(b"symbol: x\n"),
            Some("expected.query.stdout")
        );
        assert_eq!(CommandKind::Check.expected_stdout_write_path(b""), None);
    }

    fn temp_workspace(name: &str) -> PathBuf {
        let path = temp_dir(name);
        fs::create_dir_all(fixture_root(&path)).expect("fixture root");
        fs::create_dir_all(seed_root(&path)).expect("seed root");
        fs::create_dir_all(path.join("xtask")).expect("xtask dir");
        fs::write(path.join("Cargo.toml"), "[workspace]\n").expect("workspace cargo");
        path
    }

    fn temp_fixture(name: &str) -> PathBuf {
        let path = temp_dir(name);
        fs::create_dir_all(&path).expect("fixture directory");
        path
    }

    fn temp_dir(name: &str) -> PathBuf {
        let nonce = TEMP_COUNTER.fetch_add(1, Ordering::SeqCst);
        let path =
            std::env::temp_dir().join(format!("ochams-fixtures-{name}-{}-{nonce}", process::id()));
        match fs::remove_dir_all(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(error) => panic!("could not clear {}: {error}", path.display()),
        }
        path
    }

    fn write(root: &Path, rel_path: &str, content: &str) {
        write_file(&root.join(rel_path), content);
    }

    fn write_bytes(root: &Path, rel_path: &str, content: &[u8]) {
        let path = root.join(rel_path);
        fs::create_dir_all(path.parent().expect("parent")).expect("parent directory");
        fs::write(path, content).expect("write");
    }

    fn write_file(path: &Path, content: &str) {
        fs::create_dir_all(path.parent().expect("parent")).expect("parent directory");
        fs::write(path, content).expect("write");
    }
}
