use std::env;
use std::process;
use hema_minigrep::Config;


fn main() {
    let args = env::args();
    // dbg!(&args);
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // dbg!(&config);
    if let Err(e) = hema_minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
