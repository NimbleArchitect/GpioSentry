
//use futures::{Future, Stream};
//use reqwest::r#async::{Client, Decoder};
extern crate url;

//use url::{Url, ParseError};
//use url::Url;
use std::time::Instant;
//use std::thread::sleep;
// use std::thread;
// use std::fs;
use rppal::gpio::Gpio;
//use rppal::gpio::Level;
//use std::env;
//use std::mem;

use std::error::Error;
use std::collections::HashMap;

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
    let mut pin_prev_state: HashMap<u8, u8> = HashMap::new();



    for (_id, info) in contacts.iter_mut() {
        let pin_numb = info.pin as u8;
        println!("^ {}", info.pin);
        println!(">>    delay={}", info.delay);
        println!(">>     data={}", info.data);
        println!(">> location={}", info.location);
        println!(">>   method={}", info.method);
        println!(">>    state={}", info.state);
        println!(">>  trigger={}", info.trigger);
        if info.state == 254 { //use current state
            let state = gpio.get(info.pin).unwrap().read();
            //save the current detected state
            if state == rppal::gpio::Level::High {
                info.state = 1;
            } else {
                info.state = 0;
            }
        }

        //invert the current pin state to use as our previous state
        if info.state == 1 {
            pin_prev_state.insert(pin_numb, 1);
        } else {
            pin_prev_state.insert(pin_numb, 0);
        }

        }
        // if info.delay > 0 {
        //     let pin_number = &info.pin;
        //     //let mut this_pin = pin_state.get_mut(pin_number).unwrap();
        // }
    

    

    //let mut loop_count = 0;
    let time_now = Instant::now();
    //let mut old_time_now = 0;
    let mut time_delay = 0;
    let mut last_state = 0;

    loop { //main loop that never ends :)

        //work out time_delay
        let time_state = time_now.elapsed().as_millis();
//        println!("time_state: {}, last_state: {}, larger: {}", time_state, last_state, (time_state > last_state));
        time_delay = time_state - last_state;//TODO: might need to be swapped around
        last_state = time_state;
//        println!("time_delay: {}", time_delay);

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

        println!("pinloop start");
        //loop through each contact state
        for (_id, info) in contacts.iter_mut() {
            let pin_numb = &info.pin;
            let current_state = *pin_state.get(pin_numb).unwrap();
            let pin_prevstate = pin_prev_state[pin_numb]; //get old pin state
//            println!("check - pin {}, current_state: {}, prev_state: {}", info.pin, current_state, pin_prevstate);
            //if state has changed
            if pin_prevstate == current_state {
                //nothing has changed so reset the timeout and move to the next pin
//                println!("check - pin {}, timeout: {}, delay: {}", info.pin, info.timeout, info.delay);
                info.timeout = info.delay;
                continue;
            }
//            println!("check - pin {}, trigger: {}, current_state: {}", info.pin, info.trigger, current_state);    
            if info.trigger == current_state {
                //we have a trigger, Go! Go! Go!
                //compute timeout value
                if info.timeout > 0 {
//                    println!("reduce - pin {}, timeout: {}, delay: {}", info.pin, info.timeout, info.delay);
                    if (info.timeout - time_delay as i32)  > 0 {
                        info.timeout -= time_delay as i32;
                    } else {
                        info.timeout = 0;
                    }
                } 

                if info.timeout <= 0 {
                    //we have timed out in the changed state, so now we need to fire the trigger
                    println!("triggered pin {} at state {}", info.pin, current_state);
                    info.timeout = info.delay; //reset our timeout
                    //now we change state and update prevstate
                    //*pin_prev_state.get_mut(pin_numb).unwrap() = info.state;
                    let state = pin_prev_state.get_mut(pin_numb).unwrap();
                    *state = current_state;
                }

                //we dont update a prevstate value until we actually finalised the state
            }
        }

        // loop_count += 1;
        // let time_taken = time_now.elapsed().as_millis();
        // if time_taken >= 1000 {
        //     println!("run {} loops in {} ms", loop_count, time_taken);
        //     loop_count = 1;
        //     time_now = Instant::now();
        //     //break;
        // }
    }


    Ok(())

}
