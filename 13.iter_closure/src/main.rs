use std::env;
use std::process;

use iter_closure::Config;

fn main(){
    let args: Vec<String> = env::args().collect();

    let config = Config::build(args.into_iter()).unwrap_or_else(|err| {
        eprint!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = iter_closure::run(config) {
        eprint!("Application error: {e}");
        process::exit(1);
    }
}
