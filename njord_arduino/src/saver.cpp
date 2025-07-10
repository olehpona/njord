#include <saver.h>
#include <ArduinoJson.h>
#include <LittleFS.h>
#include <config.h>
#include <tuple>
#include <errors.h>
#include <messages.h>

void beginStorage(){
    if (!LittleFS.begin()){
        sendStringResponse(ERR_CODE, STORAGE_MOUNT_ERR);
        LittleFS.format();
        LittleFS.begin();
    }
}

void resetStorage(){
    LittleFS.format();
}

void dumpData(JsonDocument doc){
    File file = LittleFS.open(STORAGE_FILE, "w");
    serializeMsgPack(doc, file);
    file.close();
}

std::tuple<bool, JsonDocument> loadData(){
    bool status;
    JsonDocument doc;

    File storage = LittleFS.open(STORAGE_FILE, "r");
    DeserializationError err = deserializeMsgPack(doc, storage);
    storage.close();
    if(err.code() == DeserializationError::Ok) {
        status = true;
    }
    return {status, doc};
}

void loadStorage(){
    if (!LittleFS.exists(STORAGE_FILE) || !data.loadFile()) {
        sendStringResponse(ERR_CODE, CONFIG_LOAD_ERROR);
        data.loadDefault();
        sendStringResponse(OK_CODE, LOAD_DEFAULT_CONFIG_MSG);
    }
}
