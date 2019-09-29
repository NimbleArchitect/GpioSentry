

extern crate url;
use url::{Url, ParseError};
use std::fs;

//check if we have recieved a file or url
pub fn check_is_url(value: &String) -> bool {
    let val = value;
    if val.contains("://") {
        true
    } else {
        false
    }

}

//check if the provided url is valid
pub fn make_safe_url(value: &String) -> String {
    let val = value.to_string();

    let out = match Url::parse(&val) {
        //Ok(out) => out.to_string(),
        Ok(out) => out.to_string(),
        Err(_) => panic!("invalid url found!"),
    };
    
    out
}

//check if the provided file exists, this needs to be a full path to the executible
pub fn make_safe_file(value: &String) -> String {
    let out = value.to_string();
    let is_valid = fs::metadata(&out).is_ok();

    if is_valid == true {
        out.to_string()
    } else {
        panic!("file {} not found!", out)
    }
}
