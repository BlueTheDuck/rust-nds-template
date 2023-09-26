ifeq ($(strip $(DEVKITARM)),)
$(error "Please set DEVKITARM in your environment. export DEVKITARM=<path to>devkitARM")
endif

.PHONY: build clean cargo assets
.DEFAULT: build

# Configure here name and runner
NAME := $(shell cat Cargo.toml | sed -n -e 's/name = "\([^"]*\)".*/\1/p')
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

OUTPUT := target/armv5te-none-eabi/$(PROFILE)
_ADDFILES := -d nitrofiles

build: $(OUTPUT)/$(NAME).nds

$(OUTPUT)/$(NAME).nds: $(OUTPUT)/$(NAME)
	@echo "Creating ROM $@ ($(PROFILE))"
	@ndstool -c $@ -9 $< $(NDSTOOLFLAGS) $(_ADDFILES)
	@echo "File on $(OUTPUT)/$(NAME).nds"

$(OUTPUT)/$(NAME): cargo

cargo:
	@echo "Compiling code ($(PROFILE))"
	@cargo build $(CARGOFLAGS)


run: build
	$(RUNNER) $(OUTPUT)/$(NAME).nds

clean:
	@cargo clean
	@rm -f $(OUTPUT)/$(NAME).nds

update:
	@CARGO_NET_GIT_FETCH_WITH_CLI=true cargo update

assets:
	$(MAKE) -C assets 
