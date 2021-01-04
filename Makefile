.PHONY: all build run clean test stop start

BIN_DIR   := bin
NAME      := mpv_webrpc
BINARY    := $(BIN_DIR)/$(NAME)
SOURCES   := $(shell find src -name '*.rs')
TEMPLATES := $(shell find templates -name '*.html' -type f -newer $(BINARY))

#############################################################################

all: build

build: $(BINARY)

run: stop build start

start:
	SETTINGS=settings/settings2.json $(BINARY)

stop:
	- killall $(NAME)

clean:
	@rm -f $(BINARY)

$(BINARY): target/debug/$(NAME) $(TEMPLATES)
	- @[ -n "$(TEMPLATES)" ] && echo "Templates changed: $(TEMPLATES)"
	@mkdir -p $(BIN_DIR)
	@cp -v $< $@

target/debug/$(NAME): $(SOURCES)
	cargo --offline build

templates/index.html:
	echo "Updating template: $@

test:
	runTest.sh
