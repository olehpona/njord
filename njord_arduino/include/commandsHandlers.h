#pragma once

#include <storage.hpp>

extern GlobalStorage data;
extern CommandStorage command;

void setValueHandler();
void portsSetupHandler();
void getValueHandler();
void getConfigHandler();
void setConfigHandler();
void getDefaultConfigHandler();
void loadDefaultConfigHandler();
void setDefaultValueHandler();
void setUpdateTimeHandler();
void boardInfoHandler();

void handleCommand();