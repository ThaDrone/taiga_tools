use clap::{
    Args,
    Parser,
};



#[derive(Parser)]
pub struct CliArgs{
    #[clap(subcommand)]
    command:Arguments
}


#[derive(Parser)]
#[clap(author,version,about)]
pub enum Arguments{
    // Create an Issue on Taiga
    CreateIssue(CreateIssue),

    // / Retrieve an Issue from Taiga
    // GetIssue(GetIssue),
    // /// Update an issue on taiga
    // UpdateIssue(UpdateIssue),
   
    // /// Create a Task on Taiga
    // CreateTask(CreateTask),
    // /// Retrieve a Task from Taiga
    // GetTask(GetTask),
    // /// Update a Task on Taiga
    // UpdateTask(UpdateTask),
    
    // /// Create a UserStory on Taiga
    // CreateStory(CreateStory),
    // /// Retrieve a UserStory from Taiga
    // GetStory(GetStory),
    // /// Update a UserStory on Taiga
    // UpdateStory(UpdateStory),
    
    // /// Create a Epic on Taiga
    // CreateEpic(CreateEpic),
    // /// Retrieve a Epic from Taiga
    // GetEpic(GetEpic),
    // /// Update a Epic on Taiga
    // UpdateEpic(UpdateEpic),

    // IssueStatus
    
}

#[derive(Args,Debug)]
pub struct CreateIssue{


    #[arg(long)]
    subject:String,
    
    #[arg(long)]
    project: String,
    
    #[arg(long, default_value_t = String::from(""))]
    description: String,
    
    #[arg(long, default_value_t = String::from(""))]
    assigned_to: String,

    #[arg(long, default_value_t = String::from(""))]
    blocked_note: String,

    #[arg(long, default_value_t = String::from(""))]
    is_blocked: String,

    #[arg(long, default_value_t = String::from(""))]
    is_closed: String,

    #[arg(long, default_value_t = String::from(""))]
    milestone: String,
    
    #[arg(long, default_value_t = String::from(""))]
    status: String,

    #[arg(long, default_value_t = String::from(""))]
    severity: String,

    #[arg(long, default_value_t = String::from(""))]
    priority: String,

    #[arg(long, default_value_t = String::from(""))]
    typeid: String,

    #[arg(long, default_value_t = String::from(""))]
    tags: String,

    #[arg(long, default_value_t = String::from(""))]
    watchers: String,

}





