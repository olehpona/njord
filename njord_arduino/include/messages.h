#pragma once

#include <Arduino.h>
#include <ArduinoJson.h>

#define INITIAL_MESSAGE_DOC_SIZE 256

#define LOAD_DEFAULT_CONFIG_MSG F("loading-default-config")

#define OK_CODE F("ok")
#define ERR_CODE F("err")

String generateStringResponse(String code, String message);
String generateDocResponse(String code, JsonDocument doc);