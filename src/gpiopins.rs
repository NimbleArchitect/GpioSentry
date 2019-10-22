/// GpioSentry - Setup the initial state of the gpio pins, disables pin reset on drop
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

use std::collections::HashMap;

use crate::conf;


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
