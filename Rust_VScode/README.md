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


## <a id="Hardware">Аппаратное обеспечение</a>

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


## <a id="Embedded_Hal_Crate">Создание и настройка проекта: Embedded Hal Crate</a>

### Зависимости проекта
**Зависимости для разработки под процессор Cortex-M4:**
- Выбрать версии зависимостей **cortex-**, **embedded-hal**, **nb** для проекта можно [тут](https://github.com/orgs/rust-embedded/repositories?),
- Выбрать версию зависимости **panic-halt** можно [тут](https://github.com/korken89/panic-halt)

**Зависимости для разработки под отладочные платы stm32f4:**
- выбрать версию зависимости **stm32f4xx-hal** можно [тут](https://github.com/stm32-rs/stm32f4xx-hal)

Подключим необходимые библиотеки в файле Cargo.toml:
```toml
[package]
name = "stm32f411vet6u"
version = "0.1.0"
edition = "2021"

# Зависимости для разработки под процессор Cortex-M4
[dependencies]
cortex-m = "0.7"
cortex-m-rt = "0.6"
cortex-m-semihosting = "0.3.7"
panic-halt = "1.0.0"
nb = "1.1.0"
embedded-hal = "1.0.0"

# Пакет для разработки под отладочные платы stm32f4
[dependencies.stm32f4xx-hal]
version = "0.10"
features = ["rt", "stm32f411"]
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

### Установка целевой платформы и цели компиляции по-умолчанию
Необходимо установить целевую платформу для компилятора, это делается с помощью команды:
``` Bash
rustup target add <target>
```

- Для устройства, рассматриваемом в данном README используется следующий *target*: **thumbv7em-none-eabi**
``` Bash
rustup target add thumbv7em-none-eabi
```
Более подробно ознакомиться с выбором `target-a` для вашей архитектуры можно [тут](#Hardware).

После этого, создайте директорию `.cargo`, в корневом каталоге, а в ней файл `config`.
Содержимое файла `config`:
```
[target.thumbv7em-none-eabi]

[target.'cfg(all(target_arch = "arm", target_os = "none"))']

rustflags = ["-C", "link-arg=-Tlink.x"]

[build]
target = "thumbv7em-none-eabi"  # Cortex-M4
```

### Пример

Чтобы убедиться в работоспособности, рассмотрим простейший пример — *Hello word!* в мире микронтроллеров - моргание светодиодом.

Пример главного файла `main.rs` можно взять [тут](https://github.com/rust-embedded/cortex-m-quickstart/blob/master/src/main.rs)

Содержимое файла main.rs, рассматриваемого в контексте данного примера:

```Rust
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f4xx_hal::{
    pac,
    prelude::*,
    delay::Delay
};


#[entry]
fn main() -> ! {
    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();
    let _ = device.SYSCFG.constrain();

    let gpiod = device.GPIOD.split();

    let mut led = gpiod.pd13.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, &clocks);

    loop {
        led.toggle();
        delay.delay_ms(200u16);
    }
}
```

- Атрибут `#![no_std]` требуется для создания приложения для голого железа,

- Атрибут `#![no_main]` сообщает компилятору, что мы не используем основную функцию по умолчанию с вектором аргументов и типом возвращаемого значения,

- Атрибут `#[entry]` сообщает точку входа в программу.

### Компиляция

Скомпилировать программу в исполняемый файл:
``` Bash
cargo build --release
```
Программа скомпилируется в исполняемый файл `stm32f411vet6u`.

Для запуска программы на голом железе, нам нужен не исполняемый файл, а бинарный файл `.bin`.  
Конвертировать исполняемый файл в файл `.bin`:
``` Bash
cargo objcopy --bin stm32f411vet6u --target thumbv7em-none-eabi --release -- -O binary stm32f411vet6u.bin
```

## Загрузка прошивки
Загрузка прошивки на железо одинаково для всех пример, поэтому будет рассмотрена только для примера [Создание и настройка проекта: Embedded Hal Crate](#Embedded_Hal_Crate)

### Загрузка прошивки с помощью openOCD
Загрузка прошивки на микроконтроллер, представленной как filename.bin в простейшем виде выглядит следующим образом:
``` Bash
openocd -f board/stm32f4discovery.cfg -c "program <filename.bin> <FLASH memory regions> reset exit"
```

Загрузка прошивки на устройство, рассматриваемое в данном *README*:
``` Bash
openocd -f board/stm32f4discovery.cfg -c "program stm32f411vet6u.bin 0x08000000 reset exit"         
```

### Загрузка прошивки с помощью ST-Link
Загрузка прошивки на микроконтроллер, представленной как filename.bin в простейшем виде выглядит следующим образом:
``` Bash
st-flash write <filename.bin> <FLASH memory regions>
```

Загрузка прошивки на устройство, рассматриваемое в данном *README*:
``` Bash
st-flash --connect-under-reset write stm32f411vet6u.bin 0x08000000        
```
### Загрузка прошивки с помощью cargo-flash
Необходимо загрузить probe-rs-tools в котором содержатся probe-rs, cargo-flash и cargo-embed.

Чтобы установить последнюю версию:
``` Bash
cargo install probe-rs-tools --locked       
```

Версия инструмента загрузки *probe-rs*, рассматриваемая в данном README - **0.24.0**.  
Для проверки версии ввести:
``` Bash
probe-rs --version 
```

Загрузка прошивки на устройство, рассматриваемое в данном *README*:
``` Bash
probe-rs run --chip STM32F411VET6U --connect-under-reset ./stm32f411vet6u        
```

## Настройка VSCode

## Полезные ссылки
- [The Embedonomicon](https://docs.rust-embedded.org/embedonomicon/custom-target.html)  
- [The Embedded Rust Book](https://docs.rust-embedded.org/book/)  
- [Discovery](https://docs.rust-embedded.org/discovery/f3discovery/)  
- [Примеры](https://github.com/stm32-rs/stm32f4xx-hal/tree/master/examples) 