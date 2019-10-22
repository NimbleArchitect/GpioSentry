/// GpioSentry
///
/// Created by: NimbleArchitect
///

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
                .help("sleep this many nano-seconds between loops")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("play-fair")
                .long("play-fair")
                .help("gives up processing time at the end of every loop")
        )

        .get_matches();

    matches
}


fn show_current_state(info: &conf::PinConfig) {
    info!("^ {}", info.pin);
    info!(">>    delay={}", info.delay);
    info!(">>     data={}", info.data);
    info!(">> location={}", info.location);
    info!(">>   method={}", info.method);
    info!(">>    state={}", info.state);
    info!(">>  trigger={}", info.trigger);
}

fn get_pin_state(state_value: rppal::gpio::Level) -> u8{
    if state_value == rppal::gpio::Level::High {
        1
    } else {
        0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    //TODO: read command line arguments to set configurtaion file location
    //env_logger::init().unwrap();
    let mut config_filename = "/etc/gpio-watcher.conf";
    let mut config_loop_sleep = 0;
    let mut config_play_fair = 0;

    env_logger::from_env(Env::default().default_filter_or("info")).init();
    debug!("Start");

    //read command line arguments
    let args = check_args();

    if let Some(c) = args.value_of("config") {
        config_filename = c;
    }
    if let Some(c) = args.value_of("loop-delay") {
        println!("setting loop-delay: {}", c);
        config_loop_sleep = c.parse::<u64>().unwrap();
    }
    if args.is_present("play-fair") {
        println!("play nice is enabled");
        config_play_fair = 1;
    }
    
    //read configuration file    
    let gpio = Gpio::new()?;
    debug!("reading config file");
    //read_conf returns a hash table of the pin configuration 
    let mut pin_config = conf::read_conf(config_filename.to_string());

    info!("initilising pins..");
    let mut pin_state = gpiopins::init_pins(&gpio, &pin_config);
    debug!("finished initilising pins");
    let mut pin_prev_state: HashMap<u8, u8> = HashMap::new();


    for (_id, info) in pin_config.iter_mut() {
        let pin_numb = info.pin as u8;
        show_current_state(&info);

        if info.state == 254 { //use current state
            let state = gpio.get(info.pin).unwrap().read();
            //save the current detected pin state
            info.state = get_pin_state(state);
            debug!("live pin {} state is {}", info.pin, info.state);
        }

        //set the current pin state to use as our previous state
        if info.state == 1 {
            pin_prev_state.insert(pin_numb, 1);
        } else {
            pin_prev_state.insert(pin_numb, 0);
        }

    }
    

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
            *state = get_pin_state(logic_state);
        }

        if config_loop_sleep > 0 {
            debug!("sleeping for {} nanoseconds", config_loop_sleep);
            let sleep_time = std::time::Duration::from_nanos(config_loop_sleep);
            thread::sleep(sleep_time);
        }

        if config_play_fair == 1 {
            thread::yield_now();
        }

        debug!("pinloop start");
        //loop through each contact stategit
        for (_id, info) in pin_config.iter_mut() {
            let pin_numb = &info.pin;
            let current_state = *pin_state.get(pin_numb).unwrap();
            let pin_prevstate = pin_prev_state[pin_numb]; //get old pin state
            debug!("check - pin {}, current_state: {}, prev_state: {}", info.pin, current_state, pin_prevstate);
            //if state has changed

            //if trigger is the same as our previous state then we are testing for the state we was in,
            // this is bad so we skip every pin config in this state, the effect of this is we only procees
            // triggers that would move us into a new state
            if info.trigger == pin_prevstate {
                continue;
            }

            if pin_prevstate == current_state {
                //nothing has changed so reset the timeout and move to the next pin
                debug!("no change - pin {}, timeout: {}, delay: {}", info.pin, info.timeout, info.delay);
                info.timeout = {info.delay};
                continue;
            }
            info!("trigger - pin {}, trigger: {}, current_state: {}", info.pin, info.trigger, current_state);    
            if info.trigger == current_state {
                //we have a trigger, Go! Go! Go!
                let timeout = {info.timeout};

                //compute timeout value
                debug!("time_delay: {}", time_delay);

                if timeout > 0 {
                    debug!("reduce - pin {}, timeout: {}, delay: {}", info.pin, timeout, info.delay);
                    info.timeout = timeout - time_delay;
                    continue;
                }

                if timeout <= 0 {
                    //we have timed out in the changed state, so now we need to fire the trigger
                    debug!("triggered pin {} at state {}", info.pin, current_state);
                    info.timeout = {info.delay}; //reset our timeout
                    //now we change state and update prevstate
                    let state = pin_prev_state.get_mut(pin_numb).unwrap();
                    //TODO: need to create a cooldown value that will pause trigger for the pin until the 
                    // cooldown value has expired, this will help with double triggers on the doorbell
                    *state = {current_state};
                    debug!("pin_prev_state for pin {} is now set to {}", pin_numb, pin_prev_state[pin_numb]);

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
