use std::io::Write;
use std::process::{Command, Stdio};

fn wordfreq() -> Command {
    Command::new(env!("CARGO_BIN_EXE_wordfreq"))
}

fn stream(input: &str, args: &[&str]) -> std::process::Output {
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

fn corpus(lines: usize) -> String {
    let mut text = String::new();
    for i in 0..lines {
        text.push_str(&format!("w{i} w{i} hot\n"));
    }
    text
}

#[test]
fn streams_stdin_by_default() {
    let out = stream("a a b c\n", &["top", "3"]);
    assert!(out.status.success());
    assert_eq!(String::from_utf8(out.stdout).unwrap(), "2 a\n1 b\n1 c\n");
}

#[test]
fn dash_is_equivalent_to_the_default_stdin_source() {
    let text = corpus(64);
    let by_default = stream(&text, &["top", "10"]);
    let by_dash = stream(&text, &["top", "10", "-"]);
    assert!(by_default.status.success());
    assert!(by_dash.status.success());
    assert_eq!(by_default.stdout, by_dash.stdout);
}

#[test]
fn ranks_a_large_streamed_corpus() {
    let out = stream(&corpus(2000), &["top", "1"]);
    assert!(out.status.success());
    assert_eq!(String::from_utf8(out.stdout).unwrap(), "2000 hot\n");
}

#[cfg(unix)]
#[test]
fn broken_pipe_on_stdout_exits_cleanly() {
    let mut child = wordfreq()
        .args(["top", "1000000"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    // Close the read end so the child's stdout writes fail with EPIPE.
    drop(child.stdout.take());
    {
        let stdin = child.stdin.as_mut().unwrap();
        for i in 0..20000 {
            writeln!(stdin, "w{i} w{i} w{i}").unwrap();
        }
    }
    drop(child.stdin.take());
    let status = child.wait().unwrap();
    assert!(status.success(), "broken pipe must route to clean exit 0, got {status:?}");
}
