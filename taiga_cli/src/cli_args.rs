use clap::{
    Args,
    Parser, Subcommand,
};

// TODO #7 Create a macro that compares two structs, and checks wether the structs implement the same values

#[derive(Parser)]
pub struct MainArgs{
    
    /// Select which type of object you want to use
    #[clap(subcommand)]
    objecttype:ObjectTypes,
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
    /// Update the instane on Taiga
    Update,
    /// Delete the item on Taiga
    Delete,
}
#[derive(Args,Debug)]
pub struct IssueCmd{

    #[arg(value_enum)]
    method:Method,
    
     #[arg(long,required_if_eq_any([("method","read"),("method","update"),("method","delete")]), default_missing_value(None))] 
    id:Option<String>,

    #[arg(long, default_missing_value(None))]
    subject:Option<String>,
    
    #[arg(long, default_missing_value(None))]
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
    
    #[arg(long, default_missing_value(None))]
    status: Option<String>,

    #[arg(long, default_missing_value(None))]
    severity: Option<String>,

    #[arg(long, default_missing_value(None))]
    priority: Option<String>,

    #[arg(long, default_missing_value(None))]
    typeid: Option<String>,

    #[arg(long, default_missing_value(None))]
    tags: Option<String>,

    #[arg(long, default_missing_value(None))]
    watchers: Option<String>,

}



