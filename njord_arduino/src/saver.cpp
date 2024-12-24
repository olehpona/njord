#include <saver.h>
#include <ArduinoJson.h>
#include <SPIFFS.h>
#include <config.h>
#include <tuple>
#include <errors.h>
#include <messages.h>

void dumpData(JsonDocument doc){
    File file = SPIFFS.open(STORAGE_FILE, "w");
    serializeMsgPack(doc, file);
    file.close();
}

std::tuple<bool, JsonDocument> loadData(){
    bool status;
    JsonDocument doc;

    File storage = SPIFFS.open(STORAGE_FILE, "r");
    DeserializationError err = deserializeMsgPack(doc, storage);
    storage.close();
    if(err.code() == DeserializationError::Ok) {
        status = true;
    }
    return {status, doc};
}

void loadStorage(){
    if (!SPIFFS.exists(STORAGE_FILE) || !data.loadFile()) {
        Serial.println(CONFIG_LOAD_ERROR);
        data.loadDefault();
        Serial.println(LOAD_DEFAULT_CONFIG_MSG);
    }
}
