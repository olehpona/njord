#include <Arduino.h>
#include <board.h>
#include <storage.hpp>
#include <config.h>
#include <SPIFFS.h>
#include <ArduinoJson.h>
#include <messages.h>
#include <errors.h>

void setupBoard(){
    Serial.begin(BAUD_RATE);
    if (!SPIFFS.begin()){
        Serial.println(generateStringResponse(ERR_CODE, SPIFFS_MOUNT_ERR));
        SPIFFS.format();
        SPIFFS.begin();
    }

    setCpuFrequencyMhz(10);
}

void setupOutputs(){
    int currentPwm = 0;
    for (int i : data.port_config){
        pinMode(i, OUTPUT);
        ledcSetup(currentPwm, PWM_FREQ, PWM_RESOLUTION);
        ledcAttachPin(i, currentPwm);
        data.channels.push_back(currentPwm);
        if (currentPwm == MAX_PWM_CHANNEL_INDEX){
            break;
        }
    }
}

void writeOutputs(){
    if (data.channels.size() > 0) {
        for (int i =0 ; i< data.channels.size(); i++){
        int value = map(data.values[0], 0, 100, 0, (1 << PWM_RESOLUTION) - 1);
        ledcWrite(data.channels[i], value); 
        }
    }
}

void reloadOutputs(){
    ESP.restart();
}

JsonDocument getBoardInfo() {
    JsonDocument doc;

    doc[F("max_ports")] = MAX_PWM_CHANNEL_INDEX + 1;
    doc[F("board_name")] = ESP.getChipModel();

    return doc;
}