
//use futures::{Future, Stream};
//use reqwest::r#async::{Client, Decoder};
extern crate url;

//use url::{Url, ParseError};
//use url::Url;
use std::time::Instant;
use std::thread::sleep;
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
    
    let gpio = Gpio::new()?;
    let mut contacts = conf::read_conf("gpio-watcher.conf".to_string());
    let mut pin_state = gpiopins::init_pins(&gpio, &contacts);



    for (_id, info) in contacts.iter_mut() {
        println!("^ {}", info.pin);
        println!(">>    delay={}", info.delay);
        println!(">>     data={}", info.data);
        println!(">> location={}", info.location);
        println!(">>   method={}", info.method);
        println!(">>    state={}", info.state);
        println!(">>  trigger={}", info.trigger);
        if info.delay > 0 {
            let pin_number = &info.pin;
            //let mut this_pin = pin_state.get_mut(pin_number).unwrap();
        }
    }

    


    let mut loop_count = 0;
    let mut time_now = Instant::now();
    let mut old_time_now = 0;
    let mut time_delay = 0;

    loop { //main loop that never ends :)

        //work out time_delay
        time_delay = time_now.elapsed().as_millis();

        //loop through each registered pin based on the config file
        for (id, state) in pin_state.iter_mut() {
            //get a reference to the pin, 
            let result_pin = gpio.get(*id);
            //error if the pin number is not valid
            let this_pin = match result_pin {
                Ok(this_pin) => this_pin,
                Err(_) => panic!("invalid gpio number!"),
            };

            //read the pin state, the init function should have set it to an input pin
            let logic_state = this_pin.read();
            //convert the logic state to a simple int
            if logic_state == rppal::gpio::Level::High {
                *state = 1;
            } else {
                *state = 0;
            }
        }

        // //now we have read the state of all pins we can check against our actions
        // for (id, state) in &pin_state {
        //     println!("check - pin {} state: {}", id, state);
        // }


        //loop through each contact state
        for (_id, info) in contacts.iter_mut() {
            let current_state = *pin_state.get(&info.pin).unwrap();
            println!("check - pin {}, state: {}, prev_state: {}", info.pin, current_state, info.prevstate);
            //if state has changed
            if info.prevstate == current_state {
                //nothing has changed so reset the timeout and move to the next pin
                info.timeout = info.delay;
                continue;
            }
            if info.trigger == current_state {
                //we have a trigger, Go! Go! Go!
                //compare timeout value
                if info.timeout > 0 {
                    info.timeout -= time_delay as i32;
                } else {
                    //we have timed out in the changed state, so now we need to fire the trigger
                    print!("triggered pin {} at state {}", info.pin, current_state);
                    info.timeout = info.delay; //reset our timeout
                    info.prevstate = info.state; //now we change state and update prevstate
                    info.state = current_state;
                }

                //we dont update a prevstate value until we actually finalised the state
            }
        }

        loop_count += 1;
        let time_taken = time_now.elapsed().as_millis();
        if time_taken >= 1000 {
            println!("run {} loops in {} ms", loop_count, time_taken);
            loop_count = 1;
            time_now = Instant::now();
            //break;
        }
    }


    Ok(())

}
