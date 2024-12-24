#include <messages.h>
#include <Arduino.h>
#include <ArduinoJson.h>

String generateStringResponse(String code, String message) {
    JsonDocument doc;

    doc[F("code")] = code;
    doc[F("message")] = message;

    String res;
    serializeJson(doc, res);

    return res;
}

String generateDocResponse(String code, JsonDocument data) {
    JsonDocument doc;

    doc[F("code")] = code;
    doc[F("data")] = data;

    String res;
    serializeJson(doc, res);

    return res;
}