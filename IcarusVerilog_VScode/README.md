# Icarus-Verilog + расширения VSCode

Это памятка по созданию среды разработки прошивок под FPGA в дистрибутивах GNU Linux. Используемый дистрибутив - Manjaro Linux версии 23.1.1.

## Icarus Verilog  
*Icarus Verilog* — компилятор языка описания аппаратуры Verilog. Используется для симуляции и верификации проектов. Для загрузки используем менеджер пакетов Manjaro - *pamac*. В терминале вводим следующую команду:
```
pamac install iverilog
```
[Документация](https://steveicarus.github.io/iverilog/index.html) на *Icarus Verilog*

## Установка необходимых расширений для VSCode

### Verilog-HDL/SystemVerilog/Bluespec SystemVerilog

Для комфортной работы с Verilog нам потребуется расширение, обеспечивающее подсветку синтаксиса языка, форматирование, linting и snippets. Будем использовать **Verilog-HDL/SystemVerilog/Bluespec SystemVerilog** оно не единственное возможное и требует несколько сторонних программ для работы, но это лучшее из того что есть на сегодняшний день. Скачиваем его прямо из *marketplace* в *VSCode*:

![MarketPlace: Verilog-HDL/SystemVerilog/Bluespec SystemVerilog](srcImg/VerilogHDL_SystemVerilog_BluespecSystemVerilog.png)  

#### Syntax Highlighting  
Поддерживается подсветка синтаксиса для:
- Verilog-HDL
- SystemVerilog
- Bluespec SystemVerilog
- VHDL
- Vivado UCF constraints
- Synopsys Design Constraints
- Verilog Filelists (dot-F files)
- Tcl

Далее установим набор сторонних программ требующихся для работы расширения и произведем настройку. Настройки, которые <u>необходимо</u> применить в панели **Settings** -> **Extensions** будут помечаться следующим образом:

> **Attention:** *настройки* - **Применяемые параметры**

Открыть панель **Settings** на пункте **Extensions** можно сочетанием клавиш *Ctrl + ,*
на английской раскладке. Или перейти во вкладку **File** -> **Preference** -> **Settings** -> **Extensions**.

> **Attention:** *Verilog › Logging: Enabled* - **YES**

#### Сtags
  
Сtags даст автозаполнение, переход к определению и его просмотра, инстраляцию модулей и некоторые другие возможности. Загружаем:
```
pamac install ctags 
```

Проверям наличие *PATH environment*:
> **Attention:** *Verilog › Ctags: Path* - **ctags**

![Settings: Verilog › Linting: Linter ](srcImg/Extensions_ctags_PATH.png)


Добавляем следующие настройки в *.vscode/settings.json* в нашу рабочую область.
```json
{
    "ctags-companion.command": "ctags -R --fields=+nKz -f .vscode/.tags --langmap=SystemVerilog:+.v -R rtl /opt/uvm-1.2/src",
    "ctags-companion.readtagsEnabled": true,
}
```
Это нужно, чтобы поиск осуществлялся не только в рабочей области, но и в файлах за ее пределами (например, */opt/uvm-1.2/src* в примере выше). А readtags, позволит осуществлять быстрый поиск в больших рабочих пространствах.

#### Linting  
Линтинг — это автоматизированный процесс анализа кода и поиска потенциальных ошибок. Помимо поиска ошибок, линтер во многих случаях может исправить те самые ошибки автоматически.

Выбираем линтер:
> **Attention:** *Verilog › Linting: Linter* - **iverilog**

![Settings: Verilog › Linting: Linter ](srcImg/Extensions_Verilog_Linting_Linter.png)  


В Icarus Verilog можно указать собственные аргументы для проверки, например *-Wall*.
> **Attention:** *Verilog › Linting › Iverilog: Arguments* - **-Wall**

![Settings: Verilog › Linting › Iverilog: Arguments ](srcImg/Extensions_Verilog_Linting_Iverilog_Arguments.png)  


Список путей к директориям, которые будут использоваться при проверке *Icarus Verilog linting*.
> **Attention:** Verilog › Linting › Iverilog: Include Path - **-I <directory_path>**


По умолчанию линтер будет запускаться в каталоге рабочей области. Если включить эту опцию, то линтер будет запускаться из места, где находится файл для проверки. Важный момент, если подключены директивы include, то они должны содержать пути к файлам относительно файла для проверки.
> **Attention:** *Verilog › Linting › Iverilog: Run At File Location* - **Поставить галочку**

![Settings: Verilog › Linting › Iverilog: Run At File Location ](srcImg/Extensions_Linting_Iverilog_RunAtFileLocation.png)  


#### Language Server 
Языковой сервер - это специальная программа, работающая на сервере. Он принимает мета-состояние редактора и возвращает набор действий или инструкций. Используем языковой сервер svls, он будет уведомлять наш редактор об обнаруженных ошибках и предупреждениях.
Для загрузки используем помощник *Aur* - *yay*
```
yay -S svls
```

Проверям наличие *PATH environment*:
> **Attention:** *Verilog › Language Server › Svls: Path* - **svls**

![Settings: Verilog: Verilog › Language Server › Svls: Path](srcImg/Extensions_LanguageServer_svls_PATH.png)  


Выбираем языковой сервер:
> **Attention:** *Verilog: Language Server* - **svls**

![Settings: Verilog: Language Server ](srcImg/Extensions_LanguageServer_svls.png)  

### WaveTrace
WaveTrace — это интерактивная программа просмотра сигналов для разработчиков FPGA/RTL прямо из под *VSCode*.

![MarketPlace: WaveTrace](srcImg/WaveTrace.png)  

### Verilog Testbench Runner
Это расширение для запуска testbench-ей прямо из под *VSCode*. Оно добавляет две кнопки, которые появляются в строке заголовка любого файла Verilog, и элемент состояния, расположенный в правом нижнем углу. Оно запускает симуляцию на Icarus Verilog и результат симуляции предложит открыть в GTKWave.

![MarketPlace: Verilog Testbench Runner ](srcImg/Verilog_Testbench_Runner.png)  


### Симуляция 
Важным моментом является применить описанные ниже настройки, это необходимо для того чтобы *icarus* при компиляции добавлял модули из testbench-ей, если они лежат не в рабочей области.
> **Attention:** *Verilog: Icarus Testbench Include Pwd* - **YES**

![Settings: Verilog: Icarus Testbench Include Pwd ](srcImg/Extensions_Icarus_Testbench_Include_Pwd.png)  

Для демонстрации работы установленных нами расширений в директории *simulation/src* существует модуль *logicFnct.v*, в котором реализованы базовые логические операции и законы де Моргана. Для проверки этого модуля и демонстрации работы WaveTrace в директории *simulation/tb* есть Testbench - *logicFnct_tb.v*.

![View: Testbench and Extensions](srcImg/testbench_workSpace.png)

Видим, что подсветка синтаксиса присутствует и автодополнение работает. 


![View: Syntax highlighting and Autocompletion](srcImg/SyntaxHighlighting_Autocompletion.png)


Сверху в правом углу видим отметку зелёного треугольника - по нажатию на эту отметку создастся директория *build*, скомпилируется модуль и будет создан файл для симуляции(*filename.vcd*). Рядом с зеленой отметкой - отметка для очистки директории *build*.

![View: Run Clean panel](srcImg/runClean_panel.png)

Если все прошло успешно, то в директории *build* будем наблюдать:
```
build
├── dump.vcd
└── logicFnct_tb.v.out
```

Для просмотра временных диаграмм откроем файл *dump.vcd*, перед нами появится следующее окно:

![WaveTrace: add signal button](srcImg/waveTrace_addSignalButton.png)


Чтобы просмотреть сигналы в области вывода их необходимо сначала добавить. Сделать это можно следующим образом: кликнуть на синюю отметку *Add Signals* -> во всплывающем окне кликнуть на интересующий нас сигнал(в примере: *dut a*) -> снова кликнуть на синию отметку *Add Signals*.

![WaveTrace: add signal ](srcImg/waveTrace_addSignal.png)


Область вывода с выбранным сигналами:

![WaveTrace: result of Simulation ](srcImg/waveTrace_resSimulation.png)

Более подробно с возможностями WaveTrace можно ознакомиться на [странице](https://marketplace.visualstudio.com/items?itemName=wavetrace.wavetrace) Marketplace-a.

### Task Explorer
Для тех кто не любит запускать команды из Makefile, можно установить *Task Explorer*. Он представляет собой элемент выпадающего списка с целями из *Makefile* в боковой панели.

![MarketPlace: Task Explorer ](srcImg/Task_Explorer.png)  


Меняем *PATH environment* с *nmake* на *make*:
> **Attention:** *Verilog: Language Server* - **svls**

![Settings: Verilog: Task Explorer > Path to Programs ](srcImg/Extensions_Task_Explorer_PathToPrograms.png)   

Компиляция iverilog -s logicFnct_tb logicFnct_tb.v ../src/logicFnct.v