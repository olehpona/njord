#include <Arduino.h>
#include <storage.hpp>
#include <commandsHandlers.h>
#include <board.h>
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

void loop() {
  writeOutputs();
  readCommandFromSerial();
  if (newCommand){
    newCommand = false;
    handleCommand();
    command.clear();
  }
  boardLoop();
}