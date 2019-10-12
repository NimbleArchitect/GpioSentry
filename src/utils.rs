

extern crate url;
//use url::{Url, ParseError};
use url::Url;
use std::fs;
use subprocess::Exec;
use std::thread;


fn url_send(method: u8, url: String, data: String) {
//0 = get
//1 = post

    let client = reqwest::Client::new();
    if method == 1 {
        println!("sending post data \"{}\" to \"{}\"", data, url);
        let _res = client.post(&url)
            .body(data)
            .send()
            .expect("Failed to send request");
    } else {
        println!("calling url \"{}\"", url);
        let _res = client.get(&url)
            //.body(data)
            .send()
            .expect("Failed to send request");
    }

}

fn run_command(location: String) {
    //TODO: check that the program exists?
    println!("starting command: {}", location);
    Exec::shell(location);
}

//read spawn a seperate thread/process based on the method reqiested
// fuction never returns a value
pub fn do_action(method: u8, location: String, data: String) {

    match method {
        // Match a single value
        //get
        0 => {
            //do nothing
        },
        1 => {
            thread::spawn(|| url_send(0, location, data));
        },
        //post
        2 => {
            thread::spawn(|| url_send(1, location, data));
        },
        //exec
        3 => {
            thread::spawn(|| run_command(location));
        },

        _ => panic!("Method not implmented")
    }
}


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
