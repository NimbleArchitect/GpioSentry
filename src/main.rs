
//use futures::{Future, Stream};
//use reqwest::r#async::{Client, Decoder};
extern crate url;

use url::{Url, ParseError};
use std::time::Duration;
use std::thread::sleep;
use std::thread;
use std::fs;
//use std::env;
//use std::mem;

use std::error::Error;

mod conf;
mod utils;


fn url_send(url: String) {

    let client = reqwest::Client::new();
    let _res = client.post(&url)
        .body("ON")
        .send()
        .expect("Failed to send request");

}




fn main() -> Result<(), Box<dyn Error>> {
    
    println!("Start");
    
    let contacts = conf::read_conf();


    for (pin,info) in contacts {
        println!("^ {}", pin);
        println!(">>    delay={}", info.delay);
        println!(">>     data={}", info.data);
        println!(">> datahigh={}", info.dataHigh);
        println!(">>  datalow={}", info.dataLow);
        println!(">> location={}", info.location);
        println!(">>   method={}", info.method);
        println!(">>    state={}", info.state);
        println!(">>  trigger={}", info.trigger);
    }



    Ok(())

}
