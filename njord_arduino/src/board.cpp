#include <Arduino.h>
#include <board.h>
#include <storage.hpp>
#include <saver.h>
#include <ArduinoJson.h>
#include <messages.h>
#include <errors.h>
#include <hardware/pwm.h>
#include <Adafruit_TinyUSB.h>

void tud_suspend_cb(bool remote_wakeup_en) {
    for (int index = 0; index < data.values.size(); index++) {
        data.values[index] = 0;
    }
}

void tud_resume_cb() {
  Serial.println("USB Resumed");
}

void setupBoard(){
    Serial.begin(SERIAL_BAUD);
    TinyUSBDevice.setManufacturerDescriptor("OLEH CORPORATION");
    TinyUSBDevice.setProductDescriptor("KIBER BIDOSA 3000"); 

    if (watchdog_caused_reboot()){
        sendStringResponse(INFO_CODE, AFTER_HW_RESET);
    }

    if (digitalRead(INPUT_GPIO) && !watchdog_caused_reboot()){
        
        resetStorage();
        sendStringResponse(INFO_CODE, CLEAR_OK);

        pinMode(25, OUTPUT);
        watchdog_enable(300, 1);
        while (true) {
            digitalWrite(OUTPUT_GPIO, HIGH);
            delay(50);
            digitalWrite(OUTPUT_GPIO, LOW);
            delay(50);
        }
    }

    watchdog_disable();
    beginStorage();
}

void boardLoop(){
}

void setupOutputs(){
    int currentPwm = 0;

    uint16_t wrap = (1 << PWM_RESOLUTION) - 1;
    float divider = 125000000.0 / (clock_get_hz(clk_sys) * (wrap+1));

    for (int gpio : data.port_config){
        gpio_set_function(gpio, GPIO_FUNC_PWM);
        //TODO: implement checking of channels in use
        uint slice_num = pwm_gpio_to_slice_num(gpio);

        pwm_set_clkdiv(slice_num, divider);
        pwm_set_wrap(slice_num, wrap);

        pwm_set_enabled(slice_num, true);
        data.channels.push_back(gpio);
        if (currentPwm == MAX_PWM_CHANNEL_INDEX){
            break;
        }
        currentPwm++;
    }
}

void writeOutputs(){
    if (data.channels.size() > 0) {
        for (int i =0 ; i< data.channels.size(); i++){
            int value = map(data.values[0], 0, 100, 0, (1 << PWM_RESOLUTION) - 1);
            pwm_set_gpio_level(data.channels[i], value);
        }
    }
}

void reloadOutputs(){
    for (int gpio: data.port_config){
        gpio_set_function(gpio, GPIO_FUNC_NULL);
    }
    reset_block_num(RESET_PWM);
    unreset_block_num_wait_blocking(RESET_PWM);
    data.channels.clear();
    setupOutputs();
}

JsonDocument getBoardInfo() {
    JsonDocument doc;

    doc[F("max_ports")] = MAX_PWM_CHANNEL_INDEX + 1;
    doc[F("board_name")] = BOARD_NAME;

    return doc;
}
