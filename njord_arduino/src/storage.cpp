#include <storage.hpp>
#include <Arduino.h>
#include <ArduinoJson.h>
#include <vector>
#include <config.h>
#include <saver.h>

void GlobalStorage::dumpFile(){
    dumpData(getJson());
}

bool GlobalStorage::loadFile(){
    auto result = loadData();

    if(!std::get<0>(result)) {
      return false;
    }

    JsonDocument doc = std::get<1>(result);

    return loadJson(doc);
}

bool GlobalStorage::loadJson(JsonDocument doc){
    if(!doc.containsKey(F("ports"))){
      return false;
    }

    port_config.clear();
    default_values.clear();
    values.clear();

    JsonArray ports = doc[F("ports")].as<JsonArray>();

    for (int i = 0; i < ports.size(); i++){
        port_config.push_back(ports[i]);
    }
        
    for (int i : doc[F("default_values")].as<JsonArray>()){
        default_values.push_back(i);
    }

    //resizing default_values to ports_size
    if (default_values.size() > port_config.size()) {
        default_values.resize(port_config.size());
    } else if (default_values.size() < port_config.size()) {
        default_values.resize(port_config.size(), INITIAL_VALUE);
    }

    for (int i : default_values){
        values.push_back(i);
    }

    if (!doc.containsKey(F("update_time"))) {
        update_time = UPDATE_TIME;
    } else {
        update_time = doc[F("update_time")];
    };
    return true;  
}

void GlobalStorage::loadDefault(){
    JsonDocument doc;
    deserializeJson(doc, DEFAULT_CONFIG);
    loadJson(doc);
}

JsonDocument GlobalStorage::getJson(){
    JsonDocument doc;
    JsonArray ports = doc[F("ports")].to<JsonArray>();
    for (int i : port_config) {
        ports.add(i);
    }
    JsonArray default_values_arr = doc[F("default_values")].to<JsonArray>();

    for (int i : default_values){
        default_values_arr.add(i);
    }

    doc["update_time"] = update_time;
    
    return doc;
}

CommandStorage::~CommandStorage() {
    clear();
}

String CommandStorage::getCom() {
    return com;
}

int CommandStorage::getComLength() {
    return com.length();
}

void CommandStorage::setCom(String newCom) {
    com = newCom;
}

String CommandStorage::getDataElement(int index) {
    if (index < 0 || index >= data.size()) {
        return "";
    }
    return data[index];
}

void CommandStorage::addDataElement(String addData) {
    data.push_back(addData);
}

int CommandStorage::getDataLength() {
    return data.size();
}

bool CommandStorage::setFromJson(JsonDocument doc) {
    if (doc.containsKey(F("command"))) {
        setCom(doc[F("command")].as<String>());
        
        if (doc.containsKey(F("data"))) {
            JsonArray arr = doc[F("data")].as<JsonArray>();
            for (String element : arr) {
                addDataElement(element);
            }
        }
        return true;
    }
    
    return false;
}

void CommandStorage::clear() {
    com.clear();
    com.reserve(0);
    data.clear();
}

String CommandStorage::operator[] (int index) {
    if (index == 0) {
        return getCom();
    } else if (index > 0 && index <= getDataLength()) {
        return getDataElement(index - 1);
    } else {
        return "";
    }
}

bool CommandStorage::hasIndex(int index) {
    if (index >= 0 && index <= getDataLength()) {
        return true;
    }
    return false;
}