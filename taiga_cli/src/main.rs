mod cli_initialize;
mod cli_storage;
mod cli_args;

use clap::Parser;
use cli_args::{MainArgs, ObjectTypes};
use log::debug;

const BASE_URL:&str = "https://api.taiga.io";


pub fn main(){

    let (session, config)  = cli_initialize::initialize();

    let session = session.expect("Could not obtain session!");
    let config = config.expect("Could not load config!");

    
    let mut commands = MainArgs::parse();
    commands.execute(session, config) 
}




#[cfg(test)]
mod tests{
    use crate::{main};

    #[test]
    fn test_main(){
        main();
    }

    #[test]
    fn test_parse_args(){
        parseArgs();
    }
    
    // Test some functionality
}


