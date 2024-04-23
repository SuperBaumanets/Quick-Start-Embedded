#include "myInc.h"

void myHAL_GPIO_TogglePin (){
    HAL_GPIO_TogglePin(GPIOD, GPIO_PIN_12);
}