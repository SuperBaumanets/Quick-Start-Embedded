# CubeMX + openOCD + VSCode  

Это памятка по созданию среды разработки для микроконтроллеров семейства STM32 в дистрибутивах GNU Linux с использованием кодогенератора от ST CubeMX и OpenOCD в качестве ПО для программирования и отладки. Используемый дистрибутив - Manjaro Linux версии 23.1.1.

## Установка необходимого ПО для создания среды разработки  
Нам понадобиться установить менеджер пакетов yay:
```
pamac install yay
```

### CubeMX
Чтобы загрузить генератор кода CubeMX, введите в терминале следующую команду:
```
yay -S stm32cubemx
```
Или, если у вас возникли проблемы с установкой yay, то можно воспользоваться AUR.
```
git clone https://aur.archlinux.org/stm32cubemx.git
```

### GCC
Компилятор:
```
pamac install arm-none-eabi-gcc
```

### GDB
Отладчик:
```
pamac install arm-none-eabi-gdb
```

### openOCD
Загрузчик:
```
pamac install openocd
```

