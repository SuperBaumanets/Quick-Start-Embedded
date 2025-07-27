# Установка Arch Linux на Raspberry Pi

Это руководство по подготовки карты памяти microSD для установки Arch Linux для Raspberry Pi, начальной конфигурации и установки базовых пользовательских настроек.


## Требования
- Целевое устройство Raspberry Pi. В руководстве используется модель Raspberry Pi 4 8GB.
- Пользовательская машина с операционной системой Linux. В руководстве используется Manjaro Linux версии 25.0.5.
- Внешний USB-ридер для microSD карт.
- microSD карта. В руководстве используется microSD карта Kingston на 64GB, но подойдёт любая от 16GB.
- [Официальный образ ARM](http://os.archlinuxarm.org/os/ArchLinuxARM-rpi-4-latest.tar.gz), который будет использоваться для установки, работает на `armv7h` (32 бита).


## Подготовка microSD на ОС ArchLinux

Все шаги ниже должны выполняться под `root`.

1. Установите инструменты для разметки и форматирования:
    ```bash
    pacman -S dosfstools
    ```
2. Проверьте подключённые устройства командой `lsblk`. Вставьте microSD карту через USB-ридер в систему и снова выполните `lsblk` для определения нового устройства. В этом руководстве используется имя `sda`.
3. Начните разметку, но **убедитесь, что выбрано правильное устройство!**: 
    ```bash
    sudo fdisk /dev/sda
    ```  
4. Введите `p` - отобразит текущие разделы. Для выхода введите `q`.
5. Введите `o` - удалит все существующие разделы. Затем снова `p`, чтобы убедиться, что разделы отсутствуют.
6. Введите `n` → `p` → `1` для создания первого раздела. Сконфигурируйте первый сектор по умолчанию (Enter), укажите размер `+512M`. При запросе о подписи подтвердите удаление: `y`.
7. Введите `t` → `c` для установки типа `W95 FAT32 (LBA)`.
8. Введите `n` → `p` → `2` для второго раздела. Сконфигурируйте секторы по умолчанию (дважды Enter). При запросе о подписи подтвердите: `y`.
9. Введите `w` для записи таблицы разделов и выхода.
10. Отформатируйте разделы:
    ```bash
    mkfs.vfat /dev/sda1
    mkfs.ext4 /dev/sda2
    ```
11. Перейдите в `/mnt` (в некоторых дистрибутивах `/mount`).
12. Создайте точки монтирования и смонтируйте разделы:
    ```bash
    mkdir boot root
    mount /dev/sda1 boot
    mount /dev/sda2 root
    ```
13. Перейдите в `root`.
    ```bash
    cd root
    ```
14. Скачайте образ и распакйте скачанный архив:
    ```bash
    wget http://os.archlinuxarm.org/os/ArchLinuxARM-rpi-armv7-latest.tar.gz
    tar -xvzf ArchLinuxARM-rpi-armv7-latest.tar.gz
    ```
15. Перенесите файлы загрузчика:
    ```bash
    mv boot/* ../boot
    ```
16. Синхронизируйте данные: `sync`.
17. Выйдите из директорий и размонтируйте их:
    ```bash
    cd ..
    umount boot root
    rmdir boot root
    ```


## Первоначальная настройка

1. Вставьте microSD карту в Raspberry Pi и включите устройство
2. После загрузки появится приглашение к входу в систему

**Учётные записи по умолчанию:**  
- Пользователь: `alarm` (сокращение от "Arch Linux ARM")  
  Пароль: `alarm`  
- Суперпользователь: `root`  
  Пароль: `root`

**Дальнейшие действия:**  
3. Войдите под учётной записью `root` для выполнения следующих шагов:

1. Обновите часовой пояс: `timedatectl set-timezone Europe/Moscow`  
   Доступные часовые пояса можно проверить: `ls /usr/share/zoneinfo`.  Например для поиска `Europe/Moscow`: 
   ``` bash
   ls /usr/share/zoneinfo/Europe
   ```
2. Настройте системные часы: `timedatectl set-ntp true`.  
3. Отредактируйте локаль: `nano /etc/locale.gen`. Уберите `#` перед нужной локалью (например, `ru_RU.UTF-8`).
4. Сгенерируйте локаль: `locale-gen`.
5. Установите системную локаль: `localectl set-locale LANG=ru_RU.UTF-8`.*Замените `ru_RU.UTF-8` на выбранную в шаге 3*.
6. Задайте имя хоста: `hostnamectl set-hostname <ваше_имя>`  
7. Настройте файл hosts: `nano /etc/hosts`. Добавьте строки (замените `myhostname` на имя из шага 6):
   ```
    127.0.0.1   localhost.localdomain <myhostname> localhost
    ::1         localhost.localdomain <myhostname> localhost
   ```
8. Настройка swap-памяти (4 ГБ с низким приоритетом):
    ```bash
    fallocate -l 4096M /swapfile
    chmod 600 /swapfile
    mkswap /swapfile
    swapon /swapfile
    echo '/swapfile none swap defaults 0 0' >> /etc/fstab
    echo 'vm.swappiness=1' > /etc/sysctl.d/99-sysctl.conf
    ```
9. Подключение к интернет соединению. Подключите Raspberry Pi к сети через Ethernet-кабель или выполните подключение по сети Wi-Fi:
    1. Создайте конфигурацию интерфейса: `nano /etc/systemd/network/wlan0.network`
    2. Добавьте содержимое:
        ```ini
        [Match]
        Name=wlan0
   
        [Network]
        DHCP=yes
        ```
    3. Сгенерируйте конфиг Wi-Fi заменив <SSID> и <PASSWORD> на имя сети и пароль:
        ``` bash
        wpa_passphrase "<SSID>" "<PASSWORD>" > /etc/wpa_supplicant/wpa_supplicant-wlan0.conf
        ```
    4. Включите автоподключение: `systemctl enable wpa_supplicant@wlan0`
    5. Запустите подключение: `systemctl start wpa_supplicant@wlan0`
    6. Если потребуется отключить подключение по сети Wi-Fi выполнить:
        ```bash
        systemctl stop wpa_supplicant@wlan0
        systemctl disable wpa_supplicant@wlan0
        rm /etc/wpa_supplicant/wpa_supplicant-wlan0.conf
        rm /etc/systemd/network/wlan0.network
        ```  
10. Выполните полное обновление системы и перезагрузку:
    ```bash
    pacman-key --init
    pacman-key --populate archlinuxarm
    pacman -Syu
    reboot
    ```

## Настройка пользователя
### Предоставление группе `wheel` прав sudo
1. Установите sudo: `pacman -S sudo`
2. Откройте конфигурацию sudo: `EDITOR=nano visudo`  
3. Найдите строку:`# %wheel ALL=(ALL:ALL) ALL`. Удалите `#` в начале строки (раскомментируйте). Сохраните изменения и выйдите из nano.

### Создание нового пользователя
1. Выполните под root, заменив `логин` на желаемое имя пользователя.
    ```bash
    useradd -m -G wheel <логин>    
    passwd ваш_логин
    userdel alarm
    ```
