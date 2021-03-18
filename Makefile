ifeq ($(strip $(DEVKITARM)),)
$(error "Please set DEVKITARM in your environment. export DEVKITARM=<path to>devkitARM")
endif

.PHONY: build clean cargo
.DEFAULT: build

# Configure here name and runner
NAME := rust-nds-template
RUNNER := melonDS

# Configure here flags for cargo and ndstool
CARGOFLAGS :=
NDSTOOLFLAGS :=

# Profile selection
# Use "make DEBUG=1" for debug
ifdef DEBUG
PROFILE := debug
else
PROFILE := release
CARGOFLAGS += --release
endif

# Use "make VERBOSE=1" for verbose
ifdef VERBOSE
CARGOFLAGS += --verbose
NDSTOOLFLAGS += -vv
endif

OUTPUT := target/nds/$(PROFILE)

build: $(OUTPUT)/$(NAME).nds

$(OUTPUT)/$(NAME).nds: $(OUTPUT)/$(NAME).elf
	@echo "Creating rom ($(PROFILE))"
	@ndstool -c $@ -9 $< $(NDSTOOLFLAGS)

$(OUTPUT)/$(NAME).elf: cargo

cargo:
	@echo "Compiling code ($(PROFILE))"
	@cargo build $(CARGOFLAGS)


run: build
	$(RUNNER) $(OUTPUT)/$(NAME).nds

clean:
	@cargo clean
	@rm -f $(NAME)-*.nds
