TARGETS := ferris.img.bin

all: $(TARGETS)

# Generate a binary image from a PNG
# Omit C header
%.img.bin: %.png
	grit $< -g -gb -ftb -fh! -o $@

clean:
	rm -f $(TARGETS)
