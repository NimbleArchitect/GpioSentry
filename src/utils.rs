

extern crate url;
use url::{Url, ParseError};
use std::fs;

pub fn checkIsUrl(value: &String) -> bool {
    let val = value;
    if val.contains("://") {
        true
    } else {
        false
    }

}

pub fn makeSafeUrl(value: &String) -> String {
    let val = value.to_string();

    //let mut out = Url::parse(&val);
    let out = match Url::parse(&val) {
        //Ok(out) => out.to_string(),
        Ok(out) => out.to_string(),
        Err(_) => panic!("invalid url found!"),
    };
    
    out
}

pub fn makeSafeFile(value: &String) -> String {
    let out = value.to_string();
    let is_valid = fs::metadata(&out).is_ok();

    if is_valid == true {
        out.to_string()
    } else {
        panic!("file {} not found!", out)
    }
}
