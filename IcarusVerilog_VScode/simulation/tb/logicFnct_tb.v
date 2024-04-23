`timescale 1ns/100ps

//`include "../src/logicFnct.v"

module logicFnct_tb;

    reg [1:0] key;
    wire [9:0] led;

    logicFnct dut ( key, led );

    initial
        begin
            key = 2'b00;
            #10;
            key = 2'b01;
            #10;
            key = 2'b10;
            #10;
            key = 2'b11;
            #10;
        end
    initial
        $dumpvars;
endmodule