.PHONY: build clean cargo assets
.DEFAULT: build

# Tools
NDSTOOL ?= $(WONDERFUL_TOOLCHAIN)/thirdparty/blocksds/core/tools/ndstool/ndstool

# Configure here name and runner
NAME := $(shell cat Cargo.toml | sed -n -e 's/name = "\([^"]*\)".*/\1/p')
RUNNER := melonDS

# Configure here flags for cargo and ndstool
CARGOFLAGS :=
NDSTOOLFLAGS :=

DEBUG ?= 1

# Profile selection
# Use "make DEBUG=1" for debug
ifeq ($(DEBUG),1)
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


TARGET = armv5te-nintendo-ds-newlibeabi

OUTPUT := target/$(TARGET)/$(PROFILE)
_ADDFILES := -d nitrofiles

build: $(NAME)-$(PROFILE).nds

$(NAME)-$(PROFILE).nds: $(OUTPUT)/$(NAME).elf
	@echo "Creating ROM $@ ($(PROFILE))"
	@$(NDSTOOL) -c $@ -9 $< $(NDSTOOLFLAGS) $(_ADDFILES)
	@echo "File on $@"

$(OUTPUT)/$(NAME).elf: $(shell find src -name '*.rs')
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
