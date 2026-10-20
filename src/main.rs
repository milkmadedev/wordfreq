mod cli;
mod count;
mod error;
mod input;
mod output;
mod rank;
mod tokenize;

fn main() {
    let argv: Vec<std::ffi::OsString> = std::env::args_os().skip(1).collect();
    let cli = match cli::parse(&argv) {
        Ok(cli) => cli,
        Err(message) => error::fail(&message),
    };
    let text = match input::read(&cli.source) {
        Ok(text) => text,
        Err(message) => error::fail(&message),
    };
    let counts = count::count(tokenize::tokenize(&text));
    let ranked = rank::rank(&counts, cli.top);
    output::emit(&ranked);
}
