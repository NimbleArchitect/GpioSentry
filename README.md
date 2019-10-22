# GpioSentry

Designed for and tested to run on a raspberry pi GpioSentry runs as a service and watches GPIO pins waiting for a change in state, all pin settings are stored in a configuration file for easy access.  Programmed in rust as a learning project and for the performance benifets.

## Features

* Thousands of checks per second for a faster response
* Ability to sleep during the loop cycle
* Can yield/give up the CPU to play nice
* easy to monitor pins
* configurable delays
* built in http push/pull requests
* custom commands/scripts are also supported


## How it works

GpioSentry sits in a loop checking all gpio pins listed in the configuration file, once the current pin state has been read another loop compares the trigger state and the previous pin state looking for a change that has lasted for at least the timeout period.  Once the trigger has been verified as valid the configured action is run GpioSentry does not check for the return value of any action and will only log that the action should have fired.  http timeouts are silently ignored and only repeat if the event is triggered again.

## Why?

I'm in the process of setting up my smart home and wanted to diy my own alarm system, originally I was using python to read the gpio pins but the response time wasn't fast enough.  During the testing phase I was able to open and close the door (with a 2 inch gap) fast enough that the trigger never fired and after plenty of reading about alarm response times, I started searching for faster languages.  After settling on rust I wrote two programs one to check the door bell and another to check the open state of the door, adjusting timeous and rebuilding quickly became annoying enough that I started searching for alternative solutions.  With Google not turning up anything worthwhile this project was born :)


## Getting Started

This project is built on a raspberry pi and as a result these instructions will also describe how to build the project in a pi, it is assumed you already have the pi up and running with the latest version of rasbian installed and a recent copy of rust installed.  there are many great resources to assist you with the installation of rust and rasbian so I won't be going into details here.

## Installation

### Download

clone from GitHub
```
git clone https://github.com/NimbleArchitect/GpioSentry.git
cd GpioSentry
```


### Building

To build the release version run
```
cargo build --release
```

this takes hours to run so your better off running the following which will keep it running in the background.

```
nohup cargo build --release &
```
then to check the progress run the following command

```
cat nohup
```

## Configuration

The configuration file is a simple .ini file the section names can be anything with the condition that they must be unique, it is recommended that they describe the purpose of the check but you are free to name them however you wish.

A list of the item names that are accepted for each key follows:

* **pin**: gpio pin number using the bcm numbering format
* **state**: inital state that the pin should be during initialisation, if the pin is not in this state the trigger will fire.
* **trigger**: if the pin changes to this state we call an action event using the method and location
  The trigger can be set to:
  * high: pin read as high.
  * low: pin reads as low.
* **method**: how we respond to a trigger event.
  * none: no action, this has the effect of ignoring the event.
  * get: http get request for the value of location.
  * post: http post request to the location with the value of data passed as the request body.
  * exec: call the content of location as an executable.
* **location**: can be a command to execute or a url to connect to
* **data**: the data to send during the trigger event, currently only valid for post requests.
* **delay**: time to wait before activating the trigger, in milliseconds.  The trigger will only fire if this many milliseconds have passed.


### Example config

```
[main-light]
pin = 18
state = auto
trigger = high
method = post
location = "https://example.com/api/lightswitch"
data = ON
delay = 1
```

A full example config can be found in the **gpiosentry.conf** file.


### Command line

-c --config Sets a custom config file

--loop-delay sleep this many nano-seconds between loops

--play-fair gives up processing time at the end of every loop


##  Service setup

for the release version use this to copy the program to the users bin folder
```
sudo cp target/release/gpiosentry /usr/local/bin/
```
and finally copy the service file to the correct location
```
sudo cp gpiosentry.service /etc/systemd/system/
```

### Starting the service

To start the service run
```
sudo systemctl start gpiosentry
```
use the followomg to enable the service at boot
```
sudo systemctl enable gpiosentry
```


## Built With


## Contributing


## Versioning


## Authors

* **NimbleArchitect** - **Initial work**


## License



## Acknowledgments

The following rust crates were used in the making of this program and I want to say a massive thank you to the developers.

* [RPPAL](https://github.com/golemparts/rppal)
* [reqwest](https://github.com/seanmonstar/reqwest)
* [url](https://github.com/servo/rust-url)
* [rust-ini](https://github.com/zonyitoo/rust-ini)
* [subprocess](https://github.com/hniksic/rust-subprocess)
* [env_logger](https://github.com/sebasmagri/env_logger)
* [clap](https://github.com/clap-rs/clap)


* Hat tip to anyone whose code was used
* Inspiration
* etc
