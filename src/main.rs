use std::env;
use std::process::ExitCode;

use pixel_seed::{parse_seed, Config};

mod drawing;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let seed = match args.len() {
        2 => args[1].as_str(),
        _ => {
            eprintln!("Usage: genpix [HEX_SEED]");
            return ExitCode::from(3);
        }
    };
    let config: Config = match parse_seed(&seed[..]) {
        Ok(conf) => conf,
        Err(error) => {
            eprintln!("{:?}", error);
            return ExitCode::from(4);
        }
    };

    return match drawing::draw_from_config(&config) {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{:?}", error);
            return ExitCode::from(5);
        }
    };
}
