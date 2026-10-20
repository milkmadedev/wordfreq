use std::io::{self, Write};
use std::process;

/// Prefix every diagnostic with the binary name, per error-message-format.md.
pub fn render(message: &str) -> String {
    format!("wordfreq: {message}")
}

/// Single exit-1 renderer: message to stderr, then status 1.
pub fn fail(message: &str) -> ! {
    let mut stderr = io::stderr();
    let _ = writeln!(stderr, "{}", render(message));
    let _ = stderr.flush();
    process::exit(1)
}

/// Broken pipe on stdout routes clean per exit-routing-and-brokenpipe.md:
/// status 0, no stderr noise, no panic text.
pub fn broken_pipe() -> ! {
    process::exit(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messages_carry_the_wordfreq_prefix() {
        assert_eq!(render("boom"), "wordfreq: boom");
    }
}
