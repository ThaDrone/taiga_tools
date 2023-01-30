use std::{collections::HashMap, fmt::Pointer};
use reqwest::{header::{HeaderMap, HeaderValue}, blocking::{Client}, StatusCode, Method };
use serde_json;
use log::debug;

use crate::lib_models::Issue;




/// This is supposed to be an abstraction for the api.
/// Try however to not use these functions directly. 
/// Instead use the items in [`lib_models`]. This is a kind of "ORM". 
/// Here will be for example a Struct called `Issue`, then you can either fill out the structs and create it on Taiga using 
/// 'let my_issue = Issue{.<your data>.};
/// my_issue.create();'
pub trait TaigaRoute {
    fn url(&self) -> String;

    fn headers(&self) -> Result<HeaderMap,RouteError> {
        Ok(HeaderMap::new())
    }

    fn form(&self) -> Result<HashMap<&str, String>,RouteError>{
        Ok(HashMap::new())
    }

    fn needs_authkey() -> bool{
        true
    }

    fn method(&self) -> Method{
        Method::GET
    }
    
    /// Request method for structs implementing the `Taigaroute` trait
    /// Takes the base url (this should be fetched from a config file)
    /// TODO consider changing opt_auth_key parameter to Session
    fn request(&self,base_url:&str, opt_auth_key:&Option<String>) -> Result<serde_json::Value, RouteError>{


        let mut url = base_url.to_owned();
        url.push_str(&self.url());

        // Get the headers, insert the default headers as well
        let mut headers = self.headers()?;
        
        let key = "Content-Type";
        let val ="application/json"; 
       
         
        headers.insert(key,HeaderValue::from_static(&val));

        // match headers.insert(key,HeaderValue::from_static(&val)){ 
        //     Some(_) => (),
        //     None => return Err(RouteError::HeaderError{key: key,val:val.to_string()}),
        // }

        
        let method = self.method();
        
        let form = self.form()?;

        // let mut builder = Client::new().request(method, &url);
        let mut builder = Client::new()
            .request(method, &url)
            .headers(headers)
            .form(&form);
            
        /*
        Check if an authkey was given. If not, it will check if the Authentificate route was given.
        If it was not, it will return an error.
        We cannot pass a empty string in the bearer_auth, then the Taiga server will panic. 
        */
        let needs_key = Self::needs_authkey();

        if let (Some(key), true) = (opt_auth_key, needs_key) { 
            // Ideally this checked at the start of the function,
            // but I dont know if I can make the request builder for only the bearer auth, and later merge it into the rest. 
            builder = builder.bearer_auth(key);
            println!("####\n\n##### Auth required! {}", key)
            
        } else if let (None, true) = (opt_auth_key, needs_key) { 
            // If the opt_auth_key was None, but need_key is true, return a [`RouteError::NoAuthKey`]
            return Err(RouteError::NoAuthKey);
        };

    

        let response = builder.send();

        // Check if the status is correct
        let response = match response {
            Ok(resp) =>  resp,
            Err(e) => return Err(RouteError::Reqwest(e))
        };

        // Check if the status is correct
        // If an unexpected response was given, return the code and the url.
        match response.status(){
            
            //OK codes:
            StatusCode::OK => (),
            StatusCode::ACCEPTED => (),
            StatusCode::CREATED => (),
            
            // Not OK  
            // StatusCode::UNAUTHORIZED => return Err(RouteError::AuthentificationError),
            code => return Err(RouteError::UnexpectedResponse{code, url}) ,
        };

        // Check if the body is correct
        let body = match response.text() {
            Ok(body) => body,
            Err(_) => return Err(RouteError::BodyError),
        };

        match serde_json::from_str(&body){
            Ok(json) => Ok(json),
            Err(e) => Err(RouteError::SerError(e)),
        }


    } 
    
}
/// Enum that defines the options that can be used to authentificate to Taiga.
pub enum AuthType {
    Taiga{username:String,password:String}, 
    Github{username:String, password:String}, //TODO: #1 add github username + password login
    GithubToken(String), //TODO: #2 add github personal api token login
    // TODO #3 Add more auth methods
}

pub struct Authentificate<'a>{
    pub auth_type:&'a AuthType
}

impl<'a> TaigaRoute for Authentificate<'a>{
    fn url(&self) -> String {
        String::from("/api/v1/auth")
    }

    fn form(&self) -> Result<HashMap<&str, String>,RouteError> {
        
        let mut form: HashMap<&str, String> = HashMap::new();

        match self.auth_type {
            AuthType::Taiga{username, password} => {
                form.insert("type","normal".to_string());
                form.insert("username",username.to_owned());
                form.insert("password",password.to_owned());
            } ,
            AuthType::Github {username, password } => todo!(),
            AuthType::GithubToken(_) => todo!(),        
        };

        Ok(form)
    }

    fn needs_authkey() -> bool {
        false
    }

    fn method(&self) -> Method{
        Method::POST
    }
}

pub struct GetUserInfo<'a>{
    pub id:&'a MemberID<'a>,
}

pub enum MemberID<'a>{
    Me,
    ID(&'a String)
}

impl<'a> TaigaRoute for GetUserInfo<'a>{
    
    
    fn url(&self) -> String {   
        
        let url = String::from("/api/v1/users/{id}");

        let url = match self.id {
            MemberID::ID(id) => url.replace("{id}", &id),
            MemberID::Me => url.replace("{id}", &"me"),
        };

        url
    }


}

pub struct CreateIssue<'a>{
    pub(crate) issue:&'a Issue,
}

impl<'a> TaigaRoute for CreateIssue<'a> {
    fn url(&self) -> String {
        String::from("/api/v1/issues")
    }

    fn form(&self) -> Result<HashMap<&str, String>, RouteError>{
        let mut form: HashMap<&str, String>  = HashMap::new();

        // TODO check if this can be more efficient with borrows. Might get really messy though

        // Check if the required fields are given.
        if let None = self.issue.subject {
            return Err(RouteError::MissingField("subject".to_owned()))
        } else if let None = self.issue.project {
            return Err(RouteError::MissingField("project".to_owned()))
        }

        self.issue.subject.to_owned().and_then(|v| {form.insert("subject",v)});
        self.issue.project.to_owned().and_then(|v| {form.insert("project",v)});

        self.issue.assigned_to.to_owned().and_then(|v| {form.insert("assigned_to",v)});
        self.issue.blocked_note.to_owned().and_then(|v| {form.insert("blocked_note",v)});
        self.issue.description.to_owned().and_then(|v| {form.insert("description",v)});
        self.issue.is_blocked.to_owned().and_then(|v| {form.insert("is_blocked",v.to_string())});
        self.issue.is_closed.to_owned().and_then(|v| {form.insert("is_closed",v.to_string())});
        self.issue.milestone.to_owned().and_then(|v| {form.insert("milestone",v)});
        self.issue.status.to_owned().and_then(|v| {form.insert("status",v)});
        self.issue.severity.to_owned().and_then(|v| {form.insert("severity",v)});
        self.issue.priority.to_owned().and_then(|v| {form.insert("priority",v)});
        self.issue.typeid.to_owned().and_then(|v| {form.insert("type",v)});


        self.issue.tags.to_owned().and_then(|v| {form.insert("tags",v.join(","))});

        Ok(form)
    }

    fn method(&self) -> Method{
        Method::POST
    }

    fn headers(&self) -> Result<HeaderMap,RouteError> {
        Ok(HeaderMap::new())
    }

    fn needs_authkey() -> bool{
        true
    }

  
}

#[derive(Debug)]
pub enum RouteError{


    // Errors relating to building the request
    NoAuthKey,
    AuthentificationError,
    MissingField(String),

    HeaderError{key:String,val:String},

    // Errors relating to the response
    UnexpectedResponse{code:StatusCode, url:String},
    Reqwest(reqwest::Error),
    BodyError,

    // Serde Errors
    SerError(serde_json::Error),
}

impl std::error::Error for RouteError{}

impl std::fmt::Display for RouteError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RouteError::Reqwest(e) => write!{f,"Reqwest error {}",e}, 
            RouteError::MissingField(field) => write!(f,"Missing Field: {}",field ),
            RouteError::NoAuthKey => write!(f,"Missing Authkey!"),
            RouteError::AuthentificationError => write!(f,"Authentification Error!"),
            RouteError::HeaderError { key, val } => write!(f,"Error adding a heading. \nKey: {}\nVal:{}",key,val),
            RouteError::UnexpectedResponse { code, url } => write!(f,"Api responded unexpected. \nCode: {}\nUrl:{}",code,url),
            RouteError::BodyError => write!(f,"Could not read the body of the response."),
            RouteError::SerError(e) => write!(f,"Error serializing to serde_json: {}",e),
        }
    }
}


#[cfg(test)]
mod tests{
    use super::MemberID;


    #[test]
    fn test(){

        let id = MemberID::Me;
        
        let url = String::from("api/v1/users/{id}");

        let url = match id{
            MemberID::ID(id) => url.replace("{id}", &id),
            MemberID::Me => url.replace("{id}", &"me"),
        };

        println!("{url}");
    }

}