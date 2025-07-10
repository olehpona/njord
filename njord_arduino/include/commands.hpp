#pragma onece

#include <Arduino.h>

#define SET_VALUE_COMMAND F("set_value")
#define PORTS_SETUP_COMMAND F("ports_setup")
#define GET_VALUE_COMMAND F("get_value")
#define GET_CONFIG_COMMAND F("get_config")
#define SET_CONFIG_COMMAND F("set_config")
#define GET_DEFAULT_CONFIG_COMMAND F("get_default_config")
#define LOAD_DEFAULT_CONFIG_COMMAND F("load_default_config")
#define SET_DEFAULT_VALUE_COMMAND F("set_default_value")
#define SET_UPDATE_TIME_COMMAND F("set_update_time")
#define BOARD_INFO_COMMAND F("board_info")
#define PING_COMMAND F("ping")

enum Command {
    SET_VALUE_CMD,
    PORTS_SETUP_CMD,
    GET_VALUE_CMD,
    GET_CONFIG_CMD,
    SET_CONFIG_CMD,
    GET_DEFAULT_CONFIG_CMD,
    LOAD_DEFAULT_CONFIG_CMD,
    SET_DEFAULT_VALUE_CMD,
    SET_UPDATE_TIME_CMD,
    BOARD_INFO_CMD,
    PING_CMD
};

Command stringToCommand(String cmd);