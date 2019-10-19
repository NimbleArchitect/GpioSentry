use std::collections::HashMap;

use crate::conf;


// pub struct PinState {
//     pub state: i32, //state of the io pin
// }

// impl Default for PinState {
//     fn default () -> PinState {
//         PinState{
//             state: 0
//         }
//     }
// }


pub fn init_pins(gpio: &rppal::gpio::Gpio , pindata: &HashMap<String, conf::PinConfig>) -> HashMap<u8, u8>{

    let mut new_pins = HashMap::new();

    for (_id, info) in pindata {
        let pinnum = info.pin;
        let exists = new_pins.contains_key(&pinnum);
        if exists == false {
            //let pin_state = PinState::default();
            let result_pin = gpio.get(pinnum);
            
            let this_pin = match result_pin {
                Ok(this_pin) => this_pin,
                Err(_) => panic!("invalid gpio number!"),
            };
            let mut input_pin = this_pin.into_input();
            input_pin.set_reset_on_drop(false);

            new_pins.insert(pinnum, 0);
        }
    }
    new_pins
}
