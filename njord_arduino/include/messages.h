#pragma once

#include <Arduino.h>
#include <ArduinoJson.h>
#include <storage.hpp>

extern CommandStorage command;
extern bool newCommand;

#define LOAD_DEFAULT_CONFIG_MSG F("loading-default-config")
#define PONG_MSG F("pong")
#define CLEAR_OK F("clear-ok")
#define AFTER_HW_RESET F("after-hw-reset")

#define OK_CODE F("ok")
#define INFO_CODE F("info")
#define ERR_CODE F("err")

void sendStringResponse(String code, String message);
void sendDocResponse(String code, JsonDocument doc);

void readCommandFromSerial();