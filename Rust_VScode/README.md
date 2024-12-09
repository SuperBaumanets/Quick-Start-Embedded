# Rust Embedded

Это Quick Start языка Rust для разработки встраиваемых систем для микроконтроллеров семейства STM32 в дистрибутивах GNU Linux.  

В качестве инструментов для программирования и отладки будет использовано:
- OpenOCD
- ST-Link
- cargo-flash. 

В качестве рабочей среды будет использовано:
- VScode в качестве блокнота + терминал для использования инструментов загрузки и отладки.
- VScode как рабочая среда. 

Используемый дистрибутив - Manjaro Linux версии 23.1.1, отладочная плата STM32F411.  


## Аппаратное обеспечение

Для начала необходимо проверить, поддерживается ли конкретная модель микроконтроллера.Убедитесь, что выполняются следующие условия:

- Архитектура данного процессора поддерживается компилятором Rust. Чтобы проверить, так ли это, попробуйте найти вашу архитектуру. Сделать это можно несколькими способами:
    - В книге *The rustc book* в главе [*Platform Support*](https://doc.rust-lang.org/nightly/rustc/platform-support/thumbv7em-none-eabi.html) (более детальный вариант), 
    - Воспользоваться командой в терминале:
        ``` Console
        rustup target list
        ```

- Существуют ли *crate-ы*, предоставляющие автоматически сгенерированное *API* для доступа к периферийным устройствам. Информацию о данных *crate-ах* можно получить [тут](https://github.com/stm32-rs/stm32-rs).
    - Для работы с устройством, рассматриваемым в данном README используется следующий [*crate*](https://crates.io/crates/stm32f4).        
**Не является обязательным условием**

- Существует ли реализация инструментария [embedded hal](https://github.com/rust-embedded/awesome-embedded-rust?tab=readme-ov-file#stmicroelectronics-1).
    - Для работы с устройством, рассматриваемым в данном README используется следующий [*crate*](https://crates.io/crates/stm32f4xx-hal).   
**Не является обязательным условием**


## Установка необходимых пакетов для создания среды разработки 

Убедитесь, что у вас установлена новейшая версия Rust:

``` Bash
rustup --version
``` 
Результат:
``` Bash
rustup 1.27.1 (54dd3d00f 2024-04-24)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.83.0 (90b35a623 2024-11-26)`
``` 

### Отладчик GNU Arm Embedded Toolchain

Отладчик для чипов ARM - *arm-none-eabi-gdb*.
``` Bash
pamac install arm-none-eabi-gdb
```

Версия отладчика *arm-none-eabi-gdb*, рассматриваемая в данном README - **GNU gdb (GDB) 13.2**.  
Для проверки версии ввести:
``` Bash
arm-none-eabi-gdb -v
```

### Инструменты для программирования и отладки: openOCD и ST-Link

- Для загрузки *openOCD*:
    ``` Bash
    pamac install openocd
    ```

    Версия *openOCD*, рассматриваемая в данном README - **Open On-Chip Debugger 0.12.0**.  
    Для проверки версии ввести:
    ``` Bash
    openocd --version
    ```

- Для загрузки набора инструментов *stlink*:
    ``` Bash
    sudo pacman -Sy stlink
    ```
    
    Версия инструмента загрузки *st-flash*, рассматриваемая в данном README - **v1.7.0**.  
    Для проверки версии ввести:
    ``` Bash
    st-flash --version 
    ```

### Пакет cargo-binutils

Команды Cargo для вызова инструментов LLVM. Про использование данного инструмента можно узнать [тут](https://github.com/rust-embedded/cargo-binutils).  
Для загрузки:
``` Bash
cargo install cargo-binutils

rustup component add llvm-tools
```

## Создание и настройка проекта: Поддерживаемая архитектура


## Создание и настройка проекта: STM32 Peripheral Access Crates


## Создание и настройка проекта: Embedded Hal Crate

### Зависимости проекта
**Зависимости для разработки под процессор Cortex-M4:**
- Выбрать версии зависимостей **cortex-**, **embedded-hal**, **nb** для проекта можно [тут](https://github.com/orgs/rust-embedded/repositories?),
- Выбрать версию зависимости **panic-halt** можно [тут](https://github.com/korken89/panic-halt)

**Зависимости для разработки под отладочные платы stm32f4:**
- выбрать версию зависимости **stm32f4xx-hal** можно [тут](https://github.com/stm32-rs/stm32f4xx-hal)

Подключим необходимые библиотеки в файле Cargo.toml:
```toml
[package]
name = "stm32f411vet6"
version = "0.1.0"
edition = "2021"

# Зависимости для разработки под процессор Cortex-M4
[dependencies]
cortex-m = "0.7.7"
cortex-m-rt = "0.7.1"
cortex-m-semihosting = "0.3.7"
panic-halt = "1.0.0"
nb = "1.1.0"
embedded-hal = "1.0.0"

# Пакет для разработки под отладочные платы stm32f4
[dependencies.stm32f4xx-hal]
version = "0.20.0"
features = ["stm32f411", "rt"]
```

Библиотеке **cortex-m-rt** требуется создать файл *memory.x* в корневом каталоге проекта. В нём указывается сколько памяти имеет наше устройство и её адрес:
```sv
MEMORY
{
  /* FLASH and RAM are mandatory memory regions */
  /* Update examples/data_overflow.rs if you change these sizes. */
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K

  /* More memory regions can declared: for example this is a second RAM region */
  /* CCRAM : ORIGIN = 0x10000000, LENGTH = 8K */
}
```
Template для файла *memory.x* можно взять [тут](https://github.com/rust-embedded/cortex-m-rt/blob/master/memory.x)


## Загрузка прошивки
### Загрузка прошивки с помощью openOCD
### Загрузка прошивки с помощью ST-Link
### Загрузка прошивки с помощью cargo-flash

## Настройка VSCode

## Полезные ссылки
[The Embedonomicon](https://docs.rust-embedded.org/embedonomicon/custom-target.html)  
[The Embedded Rust Book](https://docs.rust-embedded.org/book/)  
[Discovery](https://docs.rust-embedded.org/discovery/f3discovery/)  