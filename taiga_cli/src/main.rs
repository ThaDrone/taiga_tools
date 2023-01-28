mod cli_initialize;
mod cli_storage;
mod cli_args;

use clap::Parser;
use log::debug;

const BASE_URL:&str = "https://api.taiga.io";


pub fn main(){

    let (session, config)  = cli_initialize::initialize();

    let session = session.expect("Could not obtain session!");
    let config = config.expect("Could not load config!");

    parseArgs();

}

pub fn parseArgs(){

    let args = cli_args::CliArgs::parse();

    // println!("Args: {}", );
}

#[cfg(test)]
mod tests{
    use crate::{main, parseArgs};

    #[test]
    fn test_main(){
        main();
    }

    #[test]
    fn test_parse_args(){
        parseArgs();
    }

}