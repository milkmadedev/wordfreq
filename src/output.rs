use std::io::{self, Write};

use crate::error;

/// Topmost stdout site: route write/flush failures inline per
/// exit-routing-and-brokenpipe.md. EPIPE exits clean (status 0); every
/// other failure renders the pinned message and exits 1.
pub fn emit(rows: &[(String, u64)]) {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    if let Err(err) = write_rows(&mut out, rows) {
        if err.kind() == io::ErrorKind::BrokenPipe {
            error::broken_pipe()
        } else {
            error::fail(&format!("cannot write output: {err}"))
        }
    }
}

fn write_rows<W: Write>(out: &mut W, rows: &[(String, u64)]) -> io::Result<()> {
    for (word, count) in rows {
        writeln!(out, "{count} {word}")?;
    }
    out.flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FailWriter {
        kind: io::ErrorKind,
    }

    impl FailWriter {
        fn new(kind: io::ErrorKind) -> Self {
            FailWriter { kind }
        }
    }

    impl io::Write for FailWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::from(self.kind))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct RowsOkFlushFail;

    impl io::Write for RowsOkFlushFail {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::other("flush failed"))
        }
    }

    #[test]
    fn writes_rows_as_count_word_lines() {
        let rows = [(String::from("b"), 2), (String::from("a"), 1)];
        let mut buf: Vec<u8> = Vec::new();
        write_rows(&mut buf, &rows).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "2 b\n1 a\n");
    }

    #[test]
    fn broken_pipe_write_errors_propagate_with_their_kind() {
        let mut writer = FailWriter::new(io::ErrorKind::BrokenPipe);
        let err = write_rows(&mut writer, &[(String::from("a"), 1)]).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::BrokenPipe);
    }

    #[test]
    fn other_write_errors_keep_their_kind_for_the_exit_path() {
        let mut writer = FailWriter::new(io::ErrorKind::NotFound);
        let err = write_rows(&mut writer, &[(String::from("a"), 1)]).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn flush_errors_surface_after_rows_are_written() {
        let mut writer = RowsOkFlushFail;
        let err = write_rows(&mut writer, &[(String::from("a"), 1)]).unwrap_err();
        assert_eq!(err.to_string(), "flush failed");
    }
}
