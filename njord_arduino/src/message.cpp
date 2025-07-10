#include <messages.h>
#include <Arduino.h>
#include <ArduinoJson.h>
#include <errors.h>

void sendStringResponse(String code, String message) {
    JsonDocument doc;

    doc[F("code")] = code;
    doc[F("message")] = message;

    String res;
    serializeJson(doc, res);

    Serial.println(res);
}

void sendDocResponse(String code, JsonDocument data) {
    JsonDocument doc;

    doc[F("code")] = code;
    doc[F("data")] = data;

    String res;
    serializeJson(doc, res);

    Serial.println(res);
}

void readCommandFromSerial() {
  if (Serial.available()) {
    String readed = Serial.readStringUntil('\n');
    JsonDocument doc;
    deserializeJson(doc, readed);
    if(!command.setFromJson(doc)){
      sendStringResponse(ERR_CODE,BAD_JSON_ERROR);
    };
    newCommand = true;
  }
}