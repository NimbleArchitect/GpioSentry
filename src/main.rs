
//use futures::{Future, Stream};
//use reqwest::r#async::{Client, Decoder};
extern crate url;

//use url::{Url, ParseError};
//use url::Url;
// use std::time::Duration;
// use std::thread::sleep;
// use std::thread;
// use std::fs;
use rppal::gpio::Gpio;
//use rppal::gpio::Level;
//use std::env;
//use std::mem;

use std::error::Error;

mod conf;
mod utils;
mod gpiopins;


fn url_send(url: String) {

    let client = reqwest::Client::new();
    let _res = client.post(&url)
        .body("ON")
        .send()
        .expect("Failed to send request");

}



fn main() -> Result<(), Box<dyn Error>> {
    
    println!("Start");
    
    let contacts = conf::read_conf("gpio-watcher.conf".to_string());


    for (_id, info) in &contacts {
        println!("^ {}", info.pin);
        println!(">>    delay={}", info.delay);
        println!(">>     data={}", info.data);
        println!(">> location={}", info.location);
        println!(">>   method={}", info.method);
        println!(">>    state={}", info.state);
        println!(">>  trigger={}", info.trigger);
    }

    
    let gpio = Gpio::new()?;

//    let mut time_now = Instant::now();;
    //let mut time_diff = 0;


//    let mut prev_contact_state = contact_pin.read(); 
    //let mut prev_contact_state = Level::High; 
    let mut pin_array = gpiopins::init_pins(&gpio, &contacts);

    loop { //main loop that never ends :)
        //loop through each registered pin based on the config file
        for (id, mut info) in pin_array.iter_mut() {
            let result_pin = gpio.get(*id);

            let this_pin = match result_pin {
                Ok(this_pin) => this_pin,
                Err(_) => panic!("invalid gpio number!"),
            };

            //read the pin state, the init function should have set it to an input pin
            let logic_state = this_pin.read();
            //convert the logic state to a simple int
            if logic_state == rppal::gpio::Level::High {
                info.state = 1;
            } else {
                info.state = 0;
            }

            //if the state has changed, tell the user
            if info.prev_state != info.state {
                println!("was: {}, now: {}", info.prev_state, info.state);
                //TODO: save the pin number and state to array ready for next loop
            }

            //save the state as the new previous state
            info.prev_state = info.state;
        }

        //TODO: create loop that will call the action listed for each pin

        break;
    }


    Ok(())

}
