#include <Arduino.h>
#include <ArduinoJson.h>
#include <config.h>
#include <storage.hpp>
#include <errors.h>
#include <commandsHandlers.h>
#include <board.h>
#include <SPIFFS.h>
#include <messages.h>
#include <saver.h>

bool newCommand = false;

GlobalStorage data;
CommandStorage command;

void setup() {
  setupBoard();
  loadStorage();
  setupOutputs();
}

void readCommandFromSerial() {
  if (Serial.available()) {
    String readed = Serial.readStringUntil('\n');
    JsonDocument doc;
    deserializeJson(doc, readed);
    if(!command.setFromJson(doc)){
      Serial.println(generateStringResponse(ERR_CODE, BAD_JSON_ERROR));
      return;
    };
    newCommand = true;
  }
}

void loop() {
  writeOutputs();
  readCommandFromSerial();
  if (newCommand){
    newCommand = false;
    handleCommand();
    command.clear();
  }
}