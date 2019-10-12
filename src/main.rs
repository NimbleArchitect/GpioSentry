
extern crate url;

use std::time::Instant;
use rppal::gpio::Gpio;
use std::error::Error;
use std::collections::HashMap;
use std::thread;

extern crate clap;
use clap::{App, Arg};

#[macro_use]
extern crate log;
extern crate env_logger;
use env_logger::Env;

mod conf;
mod utils;
mod gpiopins;


fn check_args() -> clap::ArgMatches<'static> {

    let matches = App::new("gpio-watcher")
        .version("0.1.0")
        .about("Watches gpio pins on a raspberry pi and reacts to changes in the pins state")
        .arg(
            Arg::with_name("config")
                .long("config")
                .short("c")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("loop-delay")
                .long("loop-delay")
                .short("s")
                .value_name("TIME")
                .help("sleep this many milli seconds between checks")
                .takes_value(true),
        )

        .get_matches();

    matches
}

fn main() -> Result<(), Box<dyn Error>> {
    //TODO: read command line arguments to set configurtaion file location
    //env_logger::init().unwrap();
    let mut config_filename = "/etc/gpio-watcher.conf";
    let mut config_loop_sleep = 0;

    env_logger::from_env(Env::default().default_filter_or("info")).init();
    debug!("Start");


    let args = check_args();
    // You can check the value provided by positional arguments, or option arguments
    if let Some(c) = args.value_of("config") {
        config_filename = c;
    }
    if let Some(c) = args.value_of("loop-delay") {
        println!("setting loop-delay: {}", c);
        config_loop_sleep = c.parse::<u32>().unwrap();
    }


    
    let gpio = Gpio::new()?;
    debug!("reading config file");
    let mut contacts = conf::read_conf(config_filename.to_string());

    info!("initilising pins..");
    let mut pin_state = gpiopins::init_pins(&gpio, &contacts);
    debug!("finished initilising pins");
    let mut pin_prev_state: HashMap<u8, u8> = HashMap::new();


    for (_id, info) in contacts.iter_mut() {
        let pin_numb = info.pin as u8;
        info!("^ {}", info.pin);
        info!(">>    delay={}", info.delay);
        info!(">>     data={}", info.data);
        info!(">> location={}", info.location);
        info!(">>   method={}", info.method);
        info!(">>    state={}", info.state);
        info!(">>  trigger={}", info.trigger);
        if info.state == 254 { //use current state
            let state = gpio.get(info.pin).unwrap().read();
            //save the current detected state
            if state == rppal::gpio::Level::High {
                debug!("live pin {} state is high", info.pin);
                info.state = 1;
            } else {
                debug!("live pin {} state is low", info.pin);
                info.state = 0;
            }
        }

        //set the current pin state to use as our previous state
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
    

    

    let mut loop_count = 0;
    let mut time_taken = 0;
    
    let time_now = Instant::now();
    let mut time_delay = 0;
    let mut last_state = 0;

    loop { //main loop that never ends :)

        //work out time_delay
        let time_state = time_now.elapsed().as_millis(); //TODO: I dont think we ever get a ms elapsed between
        // calls to this function due to release performance improvments
        debug!("time_state: {}, last_state: {}, larger: {}", time_state, last_state, (time_state > last_state));
        time_delay = (time_state - last_state) as i32;
        last_state = time_state;
        debug!("time_delay: {}", time_delay);

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

        if config_loop_sleep > 0 {
            debug!("sleeping for {} milliseconds", config_loop_sleep);
            thread::sleep_ms(config_loop_sleep);
        }

        debug!("pinloop start");
        //loop through each contact state
        for (_id, info) in contacts.iter_mut() {
            let pin_numb = &info.pin;
            let current_state = *pin_state.get(pin_numb).unwrap();
            let pin_prevstate = pin_prev_state[pin_numb]; //get old pin state
            debug!("check - pin {}, current_state: {}, prev_state: {}", info.pin, current_state, pin_prevstate);
            //if state has changed
            if pin_prevstate == current_state {
                //nothing has changed so reset the timeout and move to the next pin
                debug!("check - pin {}, timeout: {}, delay: {}", info.pin, info.timeout, info.delay);
                info.delay = info.delay;
                continue;
            }
            debug!("check - pin {}, trigger: {}, current_state: {}", info.pin, info.trigger, current_state);    
            if info.trigger == current_state {
                //we have a trigger, Go! Go! Go!
                let mut timeout = 0;

                //compute timeout value
                debug!("time_delay: {}", time_delay);
                if time_delay > 0 {
                    let pin_timeout = {info.delay};
                    debug!("pin_timeout: {}", pin_timeout);
                    timeout = pin_timeout - time_delay;
                }
                if timeout > 0 {
                    debug!("reduce - pin {}, timeout: {}, delay: {}", info.pin, timeout, info.delay);
                    info.timeout = timeout;
                }

                if timeout <= 0 {
                    //we have timed out in the changed state, so now we need to fire the trigger
                    debug!("triggered pin {} at state {}", info.pin, current_state);
                    info.timeout = info.delay; //reset our timeout
                    //now we change state and update prevstate
                    let state = pin_prev_state.get_mut(pin_numb).unwrap();
                    //TODO: need to create a cooldown value that will pause trigger for the pin until the 
                    // cooldown value has expired, this will help with double triggers on the doorbell
                    *state = current_state;
                    let changed_value = pin_prev_state.get_mut(pin_numb).unwrap();
                    debug!("pin_prev_state for pin {} changed to {}", pin_numb, changed_value);

                    //with state now saved we can call the action that has been triggered
                    utils::do_action(info.method, info.location.to_string(), info.data.to_string());
                }
                //we dont update a prevstate value until we actually finalised the state
            }
        }

        loop_count += 1;
        time_taken += time_delay;
        // let time_taken = time_now.elapsed().as_millis();
        if time_taken >= 1000 {
            debug!("run {} loops in {} ms", loop_count, time_taken);
            loop_count = 1;
            time_taken = 0;
            //break;
        }
    }


    Ok(())

}
