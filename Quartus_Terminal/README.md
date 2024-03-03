### Quartus
Для того, чтобы компилировать проект для заливки прошивку на плату нам необходим quartus. Использоваться он будет без пользовательского интерфейса(без UI) - через командную строку или в make файлах. Устанавливаем quartus, следуя шагам установки. 
```
yay quartus-free 
```

### Programm
#### Для симуляции
Компиляция
iverilog -s logicFnct_tb logicFnct_tb.v ../src/logicFnct.v
Файл для симуляции (для запуска в GTKWave или WaveTrace)
vvp -la.lst -n a.out -vcd

Компиляция кучи файлов через квартус
quartus_sh --flow compile logicFnct  (в src)
quartus_pgm -m jtag -o "p;logicFnct.sof"                                             