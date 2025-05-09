TARGETS := ferris.img.bin

GRIT := $(WONDERFUL_TOOLCHAIN)/thirdparty/blocksds/core/tools/grit/grit

all: $(TARGETS)

# Generate a binary image from a PNG
# Omit C header
%.img.bin: %.png %.grit
	$(GRIT) $< -g -gb -ftb -fh! -o $@

clean:
	rm -f $(TARGETS)
