use std::{io, error, fmt};
use std::fs;
use serde::{Deserialize};
use toml;
use serde::Serialize;

// Constants
const CFG_FILENAME:&str = "taigacli_config.toml";
const SES_FILENAME:&str = "taigacli_session.toml";

use toml::{to_string};

#[derive(Debug)]
pub enum Error{

    IOerror(io::Error),
    TomlSerError(toml::ser::Error),
    TomlDeError(toml::de::Error),

} 

impl error::Error for Error{}

impl fmt::Display for Error{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match &self{
            Error::IOerror(e) => format!("IO error, something went wrong reading or writing the file: {}",e).fmt(f),
            Error::TomlSerError(e) => format!("TOML Error, something went wrong writing TOML data: {}",e).fmt(f),
            Error::TomlDeError(e) => format!("TOML Error, something went wrong reading TOML data: {}",e).fmt(f),

        }
    }
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
    fn save(&self) -> Result<(),Error> where Self: Serialize{

        let toml_str = match to_string(self) {
            Ok(data) => data,
            Err(err) => return Err(Error::TomlSerError(err)),
        };
        
        let file_location = Self::file_location();
        match fs::write(file_location, toml_str) {
            Ok(data) => data,
            Err(err) => return Err(Error::IOerror(err)),
        };

        Ok(())
    }

    /// Generic formula load the data from a string, and return it into the struct. 
    fn load() -> Result<Self,Error> where Self: for<'a> Deserialize<'a> { 

        let file_location = Self::file_location();

        let buf = match fs::read_to_string(file_location) {
            Ok(data) => data,
            Err(err) => return Err(Error::IOerror(err)),
        };

        let data:Self = match toml::from_str(&buf) {
            Ok(data) => data,
            Err(err) => return Err(Error::TomlDeError(err)),
        };
        Ok(data)
    }

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
