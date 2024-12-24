#pragma once

#include <ArduinoJson.h>
#include <Arduino.h>  
#include <stdlib.h>
#include <vector>

struct GlobalStorage {
  std::vector<int> port_config;
  std::vector<int> values;
  std::vector<int> channels;
  std::vector<int> default_values;
  int update_time = 0;

  void dumpFile();
  bool loadFile();
  bool loadJson(JsonDocument doc);
  void loadDefault();
  JsonDocument getJson();
};


class CommandStorage {
  private:
    String com;
    std::vector<String> data;
  
  public:
    ~CommandStorage();

    String getCom();
    int getComLength();
    void setCom(String newCom);

    String getDataElement(int index);
    void addDataElement(String addData);
    int getDataLength();
    
    bool setFromJson(JsonDocument doc);

    void clear();

    String operator[] (int index);
    bool hasIndex(int index);
};
