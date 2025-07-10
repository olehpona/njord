#include <commandsHandlers.h>
#include <commands.hpp>
#include <Arduino.h>
#include <ArduinoJson.h>
#include <config.h>
#include <board.h>
#include <messages.h>
#include <errors.h>

void handleCommand(){
    switch (stringToCommand(command.getCom())) {
        case SET_VALUE_CMD:
            setValueHandler();
            break;

        case PORTS_SETUP_CMD:
            portsSetupHandler();
            break;

        case GET_VALUE_CMD:
            getValueHandler();
            break;

        case GET_CONFIG_CMD:
            getConfigHandler();
            break;

        case SET_CONFIG_CMD:
            setConfigHandler();
            break;

        case GET_DEFAULT_CONFIG_CMD:
            getDefaultConfigHandler();
            break;

        case LOAD_DEFAULT_CONFIG_CMD:
            loadDefaultConfigHandler();
            break;

        case SET_DEFAULT_VALUE_CMD:
            setDefaultValueHandler();
            break;
    
        case SET_UPDATE_TIME_CMD:
            setUpdateTimeHandler();
            break;

        case PING_CMD:
            pingHandler();
            break;

        case BOARD_INFO_CMD:
            boardInfoHandler();
            break;

        default:
            pingHandler();
            break;
    }
}

bool checkPortIndex(int port){
 if (port < data.values.size() && port >= 0) return true;
 return false;
}

bool checkValueInput(int value) {
    if ( value >= 0 && value <= 100 ) return true;
    return false;
}

void setValueHandler(){
    if (command.hasIndex(1) && command.hasIndex(2)) {
        int port = command[1].toInt();
        int value = command[2].toInt();
        if (checkPortIndex(port) && checkValueInput(value)){
            data.values[port] = value;
            sendStringResponse(OK_CODE, OK_CODE);
            return;
        }
        sendStringResponse(ERR_CODE, BAD_ARGS);
        return;
    }
    sendStringResponse(ERR_CODE, BAD_ARGS_COUNT);
}

void portsSetupHandler(){
    if (command.getDataLength() > 0){
        data.port_config.clear();
        data.default_values.clear();
        data.values.clear();
        for (int i =0; i < command.getDataLength(); i++){
            data.port_config.push_back(command.getDataElement(i).toInt());
            data.default_values.push_back(INITIAL_VALUE);
            data.values.push_back(INITIAL_VALUE);
        }
        data.dumpFile();
        sendStringResponse(OK_CODE, OK_CODE);
        reloadOutputs();
    }
    sendStringResponse(ERR_CODE, BAD_ARGS_COUNT);
}

void getValueHandler(){
    JsonDocument doc;
    JsonArray arr = doc[F("values")].to<JsonArray>();
    for (int i : data.values){
        arr.add(i);
    }
    sendDocResponse(OK_CODE, doc);
}

void getConfigHandler(){
    sendDocResponse(OK_CODE, data.getJson());
}

void setConfigHandler(){
    if (!command.hasIndex(1)) {
        sendStringResponse(ERR_CODE, BAD_ARGS_COUNT);
        return;
    }

    JsonDocument doc;

    DeserializationError err = deserializeJson(doc, command[1]);
    if (err.code() != DeserializationError::Ok){
        sendStringResponse(ERR_CODE, BAD_ARGS);
        return;
    }

    if (!data.loadJson(doc)){
        sendStringResponse(ERR_CODE, BAD_ARGS);
        return;
    };

    data.dumpFile();
    sendStringResponse(OK_CODE, OK_CODE);
    reloadOutputs();
}

void getDefaultConfigHandler(){
    JsonDocument doc;
    deserializeJson(doc, DEFAULT_CONFIG);
    sendDocResponse(OK_CODE, doc);
}

void loadDefaultConfigHandler(){
    data.loadDefault();
    data.dumpFile();
    sendStringResponse(OK_CODE, OK_CODE);
    reloadOutputs();
}

void setDefaultValueHandler(){
    if (!command.hasIndex(1) || !command.hasIndex(2)){
        sendStringResponse(ERR_CODE, BAD_ARGS_COUNT);
        return;
    }

    int port = command[1].toInt();
    int value = command[2].toInt();

    if (!checkPortIndex(port) || !checkValueInput(value)){
        sendStringResponse(ERR_CODE, BAD_ARGS);
        return;    
    }

    data.default_values[port] = value;
    data.dumpFile();
    sendStringResponse(OK_CODE, OK_CODE);  
}

void setUpdateTimeHandler(){
    if (!command.hasIndex(1)){
        sendStringResponse(ERR_CODE, BAD_ARGS_COUNT);
        return;
    }

    data.update_time = command[1].toInt();
    data.dumpFile();
    sendStringResponse(OK_CODE, OK_CODE);  
}

void boardInfoHandler(){
    sendDocResponse(OK_CODE, getBoardInfo());
}
void pingHandler(){
    sendStringResponse(OK_CODE, PONG_MSG);
}