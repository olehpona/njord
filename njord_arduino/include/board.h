#pragma once

#include <storage.hpp>
#include <ArduinoJson.h>

extern GlobalStorage data;
extern CommandStorage command;

#define SERIAL_BAUD 115200

#define MAX_PWM_CHANNEL_INDEX 15
#define PWM_RESOLUTION 10
#define OUTPUT_GPIO 25 //output led GPIO for onboard commands e.g. format storage
#define INPUT_GPIO 0 //input GPIO for onboard commands e.g. format storage

void setupBoard();
void boardLoop();

void setupOutputs();
void writeOutputs();

void reloadOutputs(); // Update pins that will be used as output


JsonDocument getBoardInfo();