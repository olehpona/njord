#include <Arduino.h>
#include <commands.hpp>

Command stringToCommand(String cmd) {
    if (cmd == SET_VALUE_COMMAND) return SET_VALUE_CMD;
    if (cmd == PORTS_SETUP_COMMAND) return PORTS_SETUP_CMD;
    if (cmd == GET_VALUE_COMMAND) return GET_VALUE_CMD;
    if (cmd == GET_CONFIG_COMMAND) return GET_CONFIG_CMD;
    if (cmd == SET_CONFIG_COMMAND) return SET_CONFIG_CMD;
    if (cmd == GET_DEFAULT_CONFIG_COMMAND) return GET_DEFAULT_CONFIG_CMD;
    if (cmd == LOAD_DEFAULT_CONFIG_COMMAND) return LOAD_DEFAULT_CONFIG_CMD;
    if (cmd == SET_DEFAULT_VALUE_COMMAND) return SET_DEFAULT_VALUE_CMD;
    if (cmd == SET_UPDATE_TIME_COMMAND) return SET_UPDATE_TIME_CMD;
    if (cmd == PING_COMMAND) return PING_CMD;
    if (cmd == BOARD_INFO_COMMAND) return BOARD_INFO_CMD;
    return PING_CMD;
}