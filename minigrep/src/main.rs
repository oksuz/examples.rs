use std::{env, process, time::Instant};
use minigrep::Config;

fn main() {
    let start = Instant::now();
    let args = env::args().collect::<Vec<String>>();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem on parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("error on application {e}");
        process::exit(1);
    }

    let end = start.elapsed();

    println!("took: {}ms", end.as_millis())
}


