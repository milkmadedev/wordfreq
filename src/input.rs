use std::fs;
use std::io::{self, Read};
use std::path::Path;

use crate::cli::Source;

pub fn read(source: &Source) -> Result<String, String> {
    match source {
        Source::Stdin => {
            let mut text = String::new();
            io::stdin()
                .read_to_string(&mut text)
                .map_err(|err| format!("cannot read standard input: {err}"))?;
            Ok(text)
        }
        Source::Path(path) => read_path(path),
    }
}

fn read_path(path: &Path) -> Result<String, String> {
    let mut text = String::new();
    fs::File::open(path)
        .and_then(|mut file| file.read_to_string(&mut text))
        .map_err(|err| format!("cannot read {}: {err}", path.display()))?;
    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("wordfreq-unit-{}-{name}", std::process::id()))
    }

    #[test]
    fn reads_a_file_source() {
        let path = temp("reads.txt");
        std::fs::write(&path, "alpha beta\n").unwrap();
        let text = read(&Source::Path(path.clone())).unwrap();
        assert_eq!(text, "alpha beta\n");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn missing_file_reports_cannot_read_with_the_path() {
        let path = temp("definitely-missing.txt");
        let _ = std::fs::remove_file(&path);
        let err = read(&Source::Path(path.clone())).unwrap_err();
        assert!(err.starts_with("cannot read "), "unexpected message: {err}");
        let expected = path.to_str().unwrap();
        assert!(err.contains(expected), "message must name the path: {err}");
    }

    #[test]
    fn reading_a_directory_fails_through_the_error_path() {
        let err = read(&Source::Path(std::env::temp_dir())).unwrap_err();
        assert!(err.starts_with("cannot read "), "unexpected message: {err}");
    }
}
