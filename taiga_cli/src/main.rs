mod cli_initialize;
mod cli_storage;
mod cli_args;

use clap::Parser;
use cli_args::{MainArgs};

const BASE_URL:&str = "https://api.taiga.io";


pub fn main(){
    let mut command = MainArgs::parse();

    main_args(&mut command)
}

pub fn main_args(command:&mut MainArgs){
    let (session, config)  = cli_initialize::initialize();

    let session = session.expect("Could not obtain session!");
    let config = config.expect("Could not load config!");

    command.execute(&session, &config);

    println!("finished")
}


#[cfg(test)]
mod tests{
    use clap::command;

    use crate::{main, cli_args::{MainArgs, IssueCmd}, main_args};

    #[test]
    fn test_main(){

        let issuecmd = IssueCmd{
            method: crate::cli_args::Method::Create,
            id: None,
            subject: Some("Testsubject!".to_string()),
            description: None,
            assigned_to: None,
            blocked_note: None,
            is_blocked: None,
            is_closed: None,
            milestone: None,
            status: None,
            severity: None,
            priority: None,
            typeid: None,
            tags: None,
            watchers: None,
        };
        
        let mut command = MainArgs{
            objecttype:crate::cli_args::ObjectTypes::Issue(issuecmd)
        };

        
        main_args(&mut command)



    }

    // Test some functionality
}


