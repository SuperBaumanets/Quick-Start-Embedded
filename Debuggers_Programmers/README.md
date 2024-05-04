# Программаторы

Это памятка по восстановлению, устранению ошибок программаторов. Также рассматривается работа с ними.
Используемый дистрибутив - Manjaro Linux версии 23.1.1, отладочная плата STM32F411.

## Обновление прошивки ST-Link

### ST-Link V2
ST-Link V2 представляет собой китайский клон ST-Link-а.

![ST-Link V2](srcImg/ST-Link-V2_updateFW.jpg)


Контроллер не оригинальный и вместо STM32F103CBT6 стоит Geehy APM32F103CBT6: 

![Geehy APM32F103CBT6](srcImg/ST-Link-V2-chip_updateFW.jpg)


### Подключение ST-Link V2 к отладочной плате STM32F411


### Проблема
![Toolchain / IDE Makefile](srcImg/st-info--probe_Fail.png)


### Обновление прошивки в ОС Windows

### Обновление прошивки в ОС Linux

### Результат 

![Toolchain / IDE Makefile](srcImg/st-info--probe_Success.png)


## Доработка ST-Link v2: добавление интерфейса вывода отладочной информации SWO и ноги Reset

https://habr.com/ru/articles/402927/

## Перепрошивка ST-Link V2 в ST-LinkV2.1


## Перепрошивка ST-Link в J-link

