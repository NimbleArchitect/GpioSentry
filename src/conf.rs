
use std::collections::HashMap;

extern crate url;
use url::{Url, ParseError};

extern crate ini;
use ini::Ini;
use crate::utils::checkIsUrl;
use crate::utils::makeSafeFile;
use crate::utils::makeSafeUrl;


pub struct PinConfig {
    pub data: String,
    pub dataHigh: String,
    pub dataLow: String,
    pub delay: i32,
    pub location: String,
    pub method: u8,
    pub state: u8,
    pub trigger: u8,
}

impl Default for PinConfig {
    fn default () -> PinConfig {
        PinConfig{
            data: "".to_string(), dataHigh: "".to_string(), dataLow: "".to_string(), delay: 0, 
            location: "".to_string(), method: 0, state: 0, trigger: 0,
        }
    }
}


fn setData(pininfo:&mut PinConfig, value:String) {
    let val = value;
    let c = val.len();
    if c > 0 {
        pininfo.data = val;
    }
}

fn setDataHigh(pininfo:&mut PinConfig, value:String) {
    let val = value;
    let c = val.len();
    if c > 0 {
        pininfo.dataHigh = val;
    }
}

fn setDataLow(pininfo:&mut PinConfig, value:String) {
    let val = value;
    let c = val.len();
    if c > 0 {
        pininfo.dataLow = val;
    }
}

fn setDelay(pininfo:&mut PinConfig, value:String) {
    let val = value.parse::<i32>().unwrap();
    pininfo.delay = val;
}

fn setLocation(pininfo:&mut PinConfig, value:String) {
    //Location could be either a file path or a weburl
    let mut val = "";
    let str_loc = value;
    //println!(">> {}", str_loc);
    let c = str_loc.len();
    if c > 0 {
        let isUrl = checkIsUrl(&str_loc);

        let val = if isUrl == true {
            let out = makeSafeUrl(&str_loc);
            out
        } else {
            let out = makeSafeFile(&str_loc);
            out
        };
        println!("{}", val);
        pininfo.location = val;
    }
}

fn setMethod(pininfo:&mut PinConfig, value:String) {
    let val = value;
    let c = val.len();
    if c > 0 {
        match val.as_ref() {
            // Match a single value
            "get" => {pininfo.method = 0},
            "post" => {pininfo.method = 1},
            "exec" => {pininfo.method = 2},
            
            _ => println!("unknown method {}", val)
        }
    }
}

fn setState(pininfo:&mut PinConfig, value:String) {
    let val = value;
    let c = val.len();
    if c > 0 {
        if val == "high" {
            pininfo.state = 1;
        } else if val == "low" {
            pininfo.state = 0;
        } else {
            panic!("invalid State value, found {} but I expeceted high or low!!", val)
        }
    }
}

fn setTrigger(pininfo:&mut PinConfig, value:String) {
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


pub fn read_conf() -> HashMap<String, PinConfig> {
    let conf_file = Ini::load_from_file("gpio-watcher.conf").unwrap();

    let section_count = conf_file.sections().count();

    let mut pin_settings = HashMap::new();
    
    for (str_pin, prop) in &conf_file {
        let pin = str_pin.as_ref().unwrap();
        let pin_number = pin.replace("pin", "");
        
        let mut pininfo = PinConfig::default();

        for (key, value) in prop {
            //println!("{:?}:{:?}", key, value);
            match key.as_ref() {
                // Match a single value
                "data" => setData(&mut pininfo, value.to_string()),
                "datahigh" => setDataHigh(&mut pininfo, value.to_string()),
                "datalow" => setDataLow(&mut pininfo, value.to_string()),
                "delay" => setDelay(&mut pininfo, value.to_string()),
                "location" => setLocation(&mut pininfo, value.to_string()),
                "method" => setMethod(&mut pininfo, value.to_string()),
                "state" => setState(&mut pininfo, value.to_string()),
                "trigger" => setTrigger(&mut pininfo, value.to_string()),
                
                _ => println!("unknown key {}", key)
            }

        }

        pin_settings.insert(pin_number, pininfo);

    }

    pin_settings

}
