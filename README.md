gpio-watcher




# Config file

```
[name] - can be any random string, generally used to descrip the pin
pin - pin number in bcm format
state - inital state that the pin should be during init
trigger - if the pin is in this state we call a trigger, can be high/low/both
method - how we respond to a trigger event none/get/post/exec
location - can be a command to execute or a url to connect too
data = the data to send during the trigger event
delay = time to wait before activating the trigger, in milliseconds
```

# Config file example

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

# Command line

* -c --config
    Sets a custom config file

* --loop-delay
    sleep this many nano-seconds between loops
    
* --play-fair
    gives up processing time at the end of every loop
