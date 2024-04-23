ITEMS := logicFnct

SOURCES := *.v
VVP := $(patsubst %,output/%,$(ITEMS))
WAVES := $(patsubst %,output/%.vcd,$(ITEMS))

.PHONY: all waves clean

all: waves

waves: $(WAVES)

$(WAVES): output/%.vcd: output/%
	$<;
	mv ./$(patsubst output/%,%.vcd,$^) $@

$(VVP): output/%: simulation/tb/%_tb.v simulation/src/$(SOURCES) output
	iverilog -g2009 -s $(ITEMS)_tb $< ../src/logicFnct.v -o $@

output:
	mkdir output

clean: output
	rm -rf output