/// GpioSentry - helper functions
///
///    Copyright (C) 2019 NimbleArchitect
///
///    This program is free software: you can redistribute it and/or modify
///    it under the terms of the GNU General Public License as published by
///    the Free Software Foundation, either version 3 of the License, or
///    (at your option) any later version.
///
///    This program is distributed in the hope that it will be useful,
///    but WITHOUT ANY WARRANTY; without even the implied warranty of
///    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
///    GNU General Public License for more details.
///
///    You should have received a copy of the GNU General Public License
///    along with this program.  If not, see <https://www.gnu.org/licenses/>.
///

extern crate url;

use url::Url;
use std::fs;
use subprocess::Exec;
use std::thread;


fn url_send(method: u8, url: String, data: String) {
    //0 = get
    //1 = post
    debug!("F:url_send:start");

    if method == 1 {
        println!("sending post data \"{}\" to \"{}\"", data, url);
        let _response = reqwest::blocking::Client::new()
            .post(url)
            .body(data)
            .send();
            // .expect("Failed to send request");
    } else {
        println!("calling url \"{}\"", url);
        let _resp = reqwest::blocking::get(url);
    }

}

fn run_command(location: String) {
    //TODO: check that the program exists?
    println!("starting command: {}", location);
    let _exit_status = Exec::cmd(location).join();
    
}

//read spawn a seperate thread/process based on the method reqiested
// fuction never returns a value
pub fn do_action(method: u8, location: String, data: String) {
    debug!("F:do_action:start");
    debug!("F:do_action:method = {}", method);

    match method {
        // Match a single value
        //get
        0 => {
            //do nothing
        },
        1 => {
            let getcall = thread::spawn(|| {
                url_send(0, location, data);
            });
            getcall.join().expect("unable to run")
        },
        //post
        2 => {
            let getcall =thread::spawn(|| {
                url_send(1, location, data);
            });
            getcall.join().expect("unable to run")
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
