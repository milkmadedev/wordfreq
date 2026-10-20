use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

const USAGE: &str = "usage: wordfreq top <n> [file|-]";

fn wordfreq() -> Command {
    Command::new(env!("CARGO_BIN_EXE_wordfreq"))
}

fn usage_error() -> String {
    format!("wordfreq: {USAGE}\n")
}

fn temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("wordfreq-cli-{}-{name}", std::process::id()))
}

fn write_corpus(name: &str, contents: &str) -> PathBuf {
    let path = temp_path(name);
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
    path
}

fn run_stdin(input: &str, args: &[&str]) -> std::process::Output {
    let mut child = wordfreq()
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn top_n_of_a_file_prints_ranked_rows() {
    let path = write_corpus("ranked.txt", "b b a c\na b\n");
    let out = wordfreq().args(["top", "2"]).arg(&path).output().unwrap();
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout), "3 b\n2 a\n");
    let _ = std::fs::remove_file(&path);
}

#[test]
fn reads_from_stdin_when_no_source_is_given() {
    let out = run_stdin("a a b\n", &["top", "2"]);
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout), "2 a\n1 b\n");
}

#[test]
fn missing_file_exits_one_with_cannot_read() {
    let missing = temp_path("definitely-not-here.txt");
    let _ = std::fs::remove_file(&missing);
    let out = wordfreq().args(["top", "2"]).arg(&missing).output().unwrap();
    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(stderr.starts_with("wordfreq: cannot read "), "stderr: {stderr}");
}

#[test]
fn directory_source_exits_one_with_cannot_read() {
    let dir = std::env::temp_dir();
    let out = wordfreq().args(["top", "2"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(stderr.starts_with("wordfreq: cannot read "), "stderr: {stderr}");
}

#[test]
fn invalid_count_exits_one_with_the_offending_value() {
    let out = wordfreq().args(["top", "many"]).output().unwrap();
    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert_eq!(stderr, "wordfreq: invalid result count 'many'\n");
}

#[test]
fn extra_positional_exits_one_with_usage() {
    let out = wordfreq()
        .args(["top", "1", "a.txt", "b.txt"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert_eq!(stderr, usage_error());
}

#[test]
fn unknown_subcommand_exits_one_with_usage() {
    let out = wordfreq().args(["histogram", "5"]).output().unwrap();
    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert_eq!(stderr, usage_error());
}

#[cfg(unix)]
#[test]
fn non_utf8_argument_exits_one_with_usage() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;
    let out = wordfreq()
        .args(["top", "1"])
        .arg(OsString::from_vec(vec![0xff, 0xfe]))
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert_eq!(stderr, usage_error());
}
