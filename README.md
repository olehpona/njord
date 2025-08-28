# njord
## Welcome
Njord project is a project that is here to add posibility to manage some of pc coolers that can't be managed from os. Firstly it is target for non pwm fans or for pwm fans that doesn't supported by hardware.
This project is trying to make this possible by creating a hardware (in my case based on rp2040 ( previously on esp32-c3 ) ) and software to get all termal info and set the fan speed based on it.
## Parts
* Njord-arduino - this is Arduino firmware for rp2040 controller that is not just implementing some standart logic but also trying to expose some board based stuff to make it possible for porting to different devices
* Njord-backend - this is implementation of all logic for getting sensors temeperature, comunication with device and making decisions about fan speed based on temperature
* Njord-gui - this is tauri program that try to make all of functions in njord-backend user friendly
## Getting started
Firstly you will need some controller that will run hardware part of njord, build it and flash it.
Secondly you can chose one of ui (for today it is only gui) and install it. Don't worry about njord-backend because it is just part of ui.
Have fun.
## My opinion
This project is maded for fun and to solve my problems (At that point, I can say that it may be discontinued because the problem it was meant to solve has disappeared.), and i don't really belive that some one will find this usefull.
