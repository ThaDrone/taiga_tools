use clap::{
    Args,
    Parser, Subcommand,
};

use crate::cli_storage::{Session, Config};

// TODO #7 Create a macro that compares two structs, and checks wether the structs implement the same values

#[derive(Parser,Debug)]
pub struct MainArgs{
    
    /// Select which type of object you want to use
    #[clap(subcommand)]
    pub objecttype:ObjectTypes,
}

impl MainArgs{
    pub fn execute(&mut self, session:Session, config:Config){

        match &self.objecttype {
            ObjectTypes::Issue(command) => command.run(session,config),
        }
    }
}

#[derive(Debug,Subcommand)]
pub enum ObjectTypes{
    Issue(IssueCmd),
    // Task(TaskCmd),
    // UserStory(UserCmd),
    // Epic(EpicCmd),
}



#[derive(Debug,clap::ValueEnum, Clone)]
pub enum Method{
    /// Create a new instance on Taiga
    Create,
    /// Retrieve the instance on Taiga
    Read,
    /// Update the instance on Taiga
    Update,
    /// Delete the instance on Taiga
    Delete,
}
#[derive(Args,Debug)]
pub struct IssueCmd{

    #[arg(value_enum)]
    method:Method,
   
    /// Required for Read, Update and Delete.
    #[arg(long,required_if_eq_any([("method","read"),("method","update"),("method","delete")]), default_missing_value(None))] 
    id:Option<String>,

    /// Subject of the Issue, Required when creating a new issue.
    #[arg(long, required_if_eq("method","create"),default_missing_value(None))]
    subject:Option<String>,
    
    /// Project ID number. Required when creating a new issue.
    #[arg(long, required_if_eq("method","create"),default_missing_value(None))]
    project: Option<String>,
    
    #[arg(long, default_missing_value(None))]
    description: Option<String>,
    
    #[arg(long, default_missing_value(None))]
    assigned_to: Option<String>,

    #[arg(long, default_missing_value(None))]
    blocked_note: Option<String>,

    #[arg(long, default_missing_value(None))]
    is_blocked: Option<String>,

    #[arg(long, default_missing_value(None))]
    is_closed: Option<String>,

    #[arg(long, default_missing_value(None))]
    milestone: Option<String>,
    
    /// Add or change the status of the ticket.
    #[arg(long, default_missing_value(None))]
    status: Option<String>,

    /// Add or change the severity of the issue.
    #[arg(long, default_missing_value(None))]
    severity: Option<String>,

    /// Add the priority of the issue.
    #[arg(long, default_missing_value(None))]
    priority: Option<String>,

    /// Use type ID.
    #[arg(long, default_missing_value(None))]
    typeid: Option<String>,

    /// Add any tags that should be added to the issue.
    #[arg(long, default_missing_value(None))]
    tags: Option<String>,

    /// Add any watchers (not sure what datatype).
    #[arg(long, default_missing_value(None))]
    watchers: Option<String>,

}

impl IssueCmd{
    pub fn run(&self, session:Session, config:Config){

        match self.method{
            Method::Create => todo!(),
            Method::Read => todo!(),
            Method::Update => todo!(),
            Method::Delete => todo!(),
        }

    }
    // TODO #9 Can this be generalized? Try to dump the IssueCMD struct into the real "Issue" model struct?
    // TODO #8 Use traits in the crud operations for the Arguments
    fn create(){
        todo!()
    }

    fn read(){
        todo!();
    }

    fn update(){
        todo!();
    }

    fn delete(){
        todo!()
    }
}


#[cfg(tests)]
mod tests{

    #[test]
    fn test_issue(){

        
    }

}