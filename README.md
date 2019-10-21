# GpioSentry

Designed for and tested to run on a raspberry pi GpioSentry runs as a service and watches GPIO pins waiting for a change in state, all pin settings are stored in a configuration file for easy access.  Programmed in rust as a learning project and for the performance benifets.

## Features

* constant checks for faster performance
* able to delay loop cycles
* can give up the CPU to play nice
* easy to monitor pins
* configurable delays
* built in http push/pull requests
* custom commands/scripts are also supported


## How it works

GpioSentry sits in a loop checking all gpio pins listed in the configuration file, once the state has been read another loop compares the trigger state and the previous pin state looking for a change that has lasted for at least the timeout period.  Once the trigger has been verified as valid the configured action is run GpioSentry does not check for the return value of any action and will only log that the action should have fired.  http timeouts are silently ignored and only repeat if the event is triggered again.

## Why?

I'm in the process of designing my smart home and wanted to diy my own alarm system, originally I was using python to read the gpio pins but the response time wasn't fast enough.  During the testing phase I was able to open and close the door (with a 2 inch gap) fast enough that the trigger never fired and after plenty of reading about alarm response times, I started searching for faster languages.  After settling on rust I wrote two programs one to check the door bell and another to check the open state of the door, adjusting timeous and rebuilding quickly became annoying enough that I started searching for alternative solutions.  With Google not turning up anything worthwhile this project was born :)


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

for the debug build run
```
cargo build
```
or too build the release version run
```
cargo build --release
```

this takes hours to run so your better off running the following which will keep it running in the background.

```
nohup cargo build --release &
```
then the progress can be checked with the following command

```
cat nohup
```

## Configuration

The configuration file is a simple .ini file the section names can be anything with the condition that they must be unique, it is recommended that they describe the purpose of the check but you are free to name them however you wish.

A list of the item names that are accepted for each key follows:

* pin - pin number in bcm format
* state - inital state that the pin should be during init
* trigger - if the pin is in this state we call a trigger, can be high/low/both
* method - how we respond to a trigger event none/get/post/exec
* location - can be a command to execute or a url to connect too
* data = the data to send during the trigger event
* delay = time to wait before activating the trigger, in milliseconds

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

### Command line

-c --config Sets a custom config file

--loop-delay sleep this many nano-seconds between loops

--play-fair gives up processing time at the end of every loop


##  Service setup

for the debug version use this to copy the program to the users bin folder
```
sudo cp target/debug/gpiosentry /usr/local/bin/
```
and use thos to copy the release build
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

