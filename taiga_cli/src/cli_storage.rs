use std::{io};
use std::fs;
use serde::{Deserialize, ser};
use toml;
use serde::Serialize;

// Constants
const CFG_FILENAME:&str = "taigacli_config.toml";
const SES_FILENAME:&str = "taigacli_session.toml";

use toml::{to_string};


pub enum LocalStorageErrors{

    IOerror(io::Error),
    TOMLerror(toml::ser::Error) 

}

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub auth_key: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    project_id: String,
}

impl LocalStorage for Session {
    /// Returns the file location of this [`Session`].
    fn file_location() -> String {
        SES_FILENAME.to_string()
    }
}

impl LocalStorage for Config {
    
    /// Returns the file location of this [`Config`].
    fn file_location() -> String {
        CFG_FILENAME.to_string()
    }

}

pub trait LocalStorage {

    /// Generic formula to save the data inside the struct to a file. 
    fn save(&self) -> Result<(),()> where Self: Serialize{
        let toml_str = to_string(self).unwrap();
        let file_location = Self::file_location();
        fs::write(file_location, toml_str).unwrap();

        Ok(())
    }

    /// Generic formula load the data from a string, and return it into the struct. 
    fn load() -> Option<Self> where Self: for<'a> Deserialize<'a> { 
        let file_location = Self::file_location();
        let toml_str = fs::read_to_string(file_location).unwrap();
        let data:Self = toml::from_str(&toml_str).unwrap();
        Some(data)
    }
    // fn load<T:LocalStorage>() -> Result<T, ()>;

    fn file_location() -> String;
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_localstorage(){

        let keys = Session{auth_key:"1234".to_string()};
        keys.save().unwrap();

    }
}
