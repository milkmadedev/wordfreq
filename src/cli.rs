use std::ffi::OsString;
use std::path::PathBuf;

pub const USAGE: &str = "usage: wordfreq top <n> [file|-]";

#[derive(Debug, PartialEq)]
pub enum Source {
    Stdin,
    Path(PathBuf),
}

#[derive(Debug)]
pub struct Cli {
    pub top: usize,
    pub source: Source,
}

pub fn parse(args: &[OsString]) -> Result<Cli, String> {
    if args.len() > 3 {
        return Err(usage());
    }
    if str_at(args, 0)?.as_str() != "top" {
        return Err(usage());
    }
    let raw = str_at(args, 1)?;
    let top = raw
        .parse::<usize>()
        .map_err(|_| format!("invalid result count '{raw}'"))?;
    let source = match args.get(2) {
        None => Source::Stdin,
        Some(_) => match str_at(args, 2)?.as_str() {
            "-" => Source::Stdin,
            path => Source::Path(PathBuf::from(path)),
        },
    };
    Ok(Cli { top, source })
}

fn usage() -> String {
    USAGE.to_string()
}

fn str_at(args: &[OsString], index: usize) -> Result<String, String> {
    match args.get(index) {
        Some(arg) => arg.to_str().map(String::from).ok_or_else(usage),
        None => Err(usage()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<OsString> {
        list.iter().map(OsString::from).collect()
    }

    #[test]
    fn parses_top_with_a_file_path() {
        let cli = parse(&args(&["top", "3", "corpus.txt"])).unwrap();
        assert_eq!(cli.top, 3);
        assert_eq!(cli.source, Source::Path(PathBuf::from("corpus.txt")));
    }

    #[test]
    fn dash_selects_standard_input() {
        let cli = parse(&args(&["top", "5", "-"])).unwrap();
        assert_eq!(cli.source, Source::Stdin);
    }

    #[test]
    fn no_source_argument_defaults_to_standard_input() {
        let cli = parse(&args(&["top", "5"])).unwrap();
        assert_eq!(cli.source, Source::Stdin);
    }

    #[test]
    fn missing_count_is_a_usage_error() {
        assert_eq!(parse(&args(&["top"])).unwrap_err(), USAGE);
        assert_eq!(parse(&args(&[])).unwrap_err(), USAGE);
    }

    #[test]
    fn unparsable_count_reports_the_offending_argument() {
        assert_eq!(parse(&args(&["top", "many"])).unwrap_err(), "invalid result count 'many'");
    }

    #[test]
    fn negative_counts_are_not_usize() {
        assert_eq!(parse(&args(&["top", "-1"])).unwrap_err(), "invalid result count '-1'");
    }

    #[test]
    fn extra_positional_is_a_usage_error() {
        assert_eq!(parse(&args(&["top", "1", "a.txt", "b.txt"])).unwrap_err(), USAGE);
    }

    #[cfg(unix)]
    #[test]
    fn non_utf8_argument_is_a_usage_error() {
        use std::os::unix::ffi::OsStringExt;
        let mut list = args(&["top", "1"]);
        list.push(OsString::from_vec(vec![0xff, 0xfe]));
        assert_eq!(parse(&list).unwrap_err(), USAGE);
    }
}
