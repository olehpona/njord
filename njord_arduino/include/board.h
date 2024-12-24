#pragma once

#include <storage.hpp>
#include <ArduinoJson.h>

extern GlobalStorage data;
extern CommandStorage command;

#define MAX_PWM_CHANNEL_INDEX 5
#define PWM_FREQ 1000
#define PWM_RESOLUTION 14

void setupBoard();

void setupOutputs();
void writeOutputs();

void reloadOutputs(); // Update pins that will be used as output

JsonDocument getBoardInfo();