
use std::collections::HashMap;

extern crate url;
//use url::{Url, ParseError};
//use url::Url;

extern crate ini;
use ini::Ini;
use crate::utils::check_is_url;
use crate::utils::make_safe_file;
use crate::utils::make_safe_url;


pub struct PinConfig {
    pub data: String,
    pub delay: i32,
    pub label: String,
    pub location: String,
    pub method: u8,
    pub pin: u8,
    pub state: u8,
    pub trigger: u8,
    pub timeout: i32
}

impl Default for PinConfig {
    fn default () -> PinConfig {
        PinConfig{
            data: "".to_string(), delay: 0, label: "".to_string(), location: "".to_string(), 
            method: 0, pin: 255, state: 254, trigger: 0, timeout: 0
        }
    }
}

// fill in the struct item data from provided value
fn set_data(pininfo:&mut PinConfig, value:String) {
    let val = value;
    let c = val.len();
    if c > 0 {
        pininfo.data = val;
    }
}

// fill in the struct item delay from provided value
fn set_delay(pininfo:&mut PinConfig, value:String) {
    let val = value.parse::<i32>().unwrap();
    pininfo.delay = val;
}

//fill in the struct item location from provided value
// this can be either a file path or url
fn set_location(pininfo:&mut PinConfig, value:String) {
    let str_loc = value;

    let c = str_loc.len();
    if c > 0 {
        let is_url = check_is_url(&str_loc);

        let val = if is_url == true {
            make_safe_url(&str_loc)
        } else {
            make_safe_file(&str_loc)
        };

        pininfo.location = val;
    }
}

// fill in the struct item method from provided value
fn set_method(pininfo:&mut PinConfig, value:String) {
    let val = value;
    let c = val.len();
    if c > 0 {
        match val.as_ref() {
            // Match a single value
            "get" => {pininfo.method = 1},
            "post" => {pininfo.method = 2},
            "exec" => {pininfo.method = 3},
            
            _ => panic!("invalid method found, method {} was found but I expected get, post or exec", val)
        }
    }
}

// fill in the struct item state from provided value
fn set_state(pininfo:&mut PinConfig, value:String) {
    let val = value;
    let c = val.len();
    if c > 0 {
        if val == "high" {
            pininfo.state = 1;
        } else if val == "low" {
            pininfo.state = 0;
        } else if val == "auto" {
            pininfo.state = 254;
        } else {
            panic!("invalid State value, found {} but I expeceted high or low!!", val)
        }
    }
}

// fill in the struct item trigger from provided value
fn set_trigger(pininfo:&mut PinConfig, value:String) {
    let val = value;
    let c = val.len();
    if c > 0 {
        if val == "high" {
            pininfo.trigger = 1;
        } else if val == "low" {
            pininfo.trigger = 0;
        } else {
            panic!("invalid trigger value, found {} but I expeceted high or low!!", val)
        }
    }
}


// fill in the struct item label from provided value
fn set_pin_label(pininfo:&mut PinConfig, value:String) {
    let val = value;
    let c = val.len();
    if c > 0 {
        pininfo.label = val;
    }
}

// fill in the struct item pin from provided value
fn set_pin_number(pininfo:&mut PinConfig, value:String) {
    let val = value.parse::<u8>().unwrap();
    pininfo.pin = val;
}

//read configuration file into hash table filling in struct as we go
pub fn read_conf(filename:String) -> HashMap<String, PinConfig> {
    let conf_file = Ini::load_from_file(filename).unwrap();

    let section_count = conf_file.sections().count();
    if section_count <= 0 {
        //no secions have been returned, it must be an invalid file so we quit early
        panic!("Invalid config file!")
    }

    let mut array_index = 0;
    let mut pin_settings = HashMap::new();
    let mut pin_high: HashMap<u8, i32> = HashMap::new();
    let mut pin_low: HashMap<u8, i32> = HashMap::new();
    //loop through each section of the ini file
    for (str_pin, prop) in &conf_file {

        let label = str_pin.as_ref().unwrap();
        //remove the word pin leaving just the number as a string, 
        // this is used later as the hash table key
        debug!("loading section: {}", label);
        //let pin_number = pin.replace("pin", "");
        //create empty structure
        let mut pininfo = PinConfig::default();

        //TODO: we still need to verify that we have recieved a valid default set of new values
        // from the ini file
        for (key, value) in prop {
            
            match key.as_ref() {
                // Match each key, and call a function to pupulate the item in the pininfo struct
                "pin" => set_pin_number(&mut pininfo, value.to_string()),
                "data" => set_data(&mut pininfo, value.to_string()),
                "delay" => set_delay(&mut pininfo, value.to_string()),
                "location" => set_location(&mut pininfo, value.to_string()),
                "method" => set_method(&mut pininfo, value.to_string()),
                "state" => set_state(&mut pininfo, value.to_string()),
                "trigger" => set_trigger(&mut pininfo, value.to_string()),
                
                _ => panic!("unexpected key found, {} is unknown!", key)
            }
        }
        set_pin_label(&mut pininfo, label.to_string());
        //by this point the struct should have been filled in :)
        // so we add the struct to the hash table
        let pin_trigger = {pininfo.trigger};
        let pin_number = {pininfo.pin};
        let trigger_time = {pininfo.delay};

        array_index += 1;
        pin_settings.insert(array_index.to_string(), pininfo);
        if pin_trigger == 1 {
            pin_high.insert(pin_number, trigger_time);
        } else {
            pin_low.insert(pin_number, trigger_time);
        }
    }

    //work out how to add missing triggers from the pair
    for (_id, pininfo) in &pin_settings {
        let pin_number = pininfo.pin;

        debug!("*Pin {}", pin_number);
        debug!(" check high");
        let high = if pin_high.contains_key(&pin_number) == true {
            debug!("  found");
            *pin_high.get_mut(&pin_number).unwrap()
        } else {
            debug!("  missing");
            0
        };
        debug!("  value {}", high);
        debug!(" check low");
        let low = if pin_low.contains_key(&pin_number) == true {
            debug!("  found");
            *pin_low.get_mut(&pin_number).unwrap()
        } else {
            debug!("  missing");
            0
        };
        debug!("  value {}", low);

        if pin_high.contains_key(&pin_number) == true {
            if pin_low.contains_key(&pin_number) == true {
                debug!("removing pair");
                pin_low.remove(&pin_number);
                pin_high.remove(&pin_number);
            }
        }
    }

    debug!("checking remaining high triggers");
    for (id, timeout) in pin_high {
        if timeout >= 1 {
            info!("missing trigger pin {}", id);
            array_index += 1;
            let mut pininfo = PinConfig::default();
            pininfo.pin = id;
            pininfo.trigger = 0;
            pininfo.delay = timeout;
            pin_settings.insert(array_index.to_string(), pininfo);
        }
    }

    debug!("checking remaining high triggers");
    for (id, timeout) in pin_low {
        if timeout >= 1 {
            info!("missing trigger for pin {}", id);
            array_index += 1;
            let mut pininfo = PinConfig::default();
            pininfo.pin = id;
            pininfo.trigger = 1;
            pininfo.delay = timeout;
            pin_settings.insert(array_index.to_string(), pininfo);
        }
    }

    pin_settings

}
