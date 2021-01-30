.PHONY: all build run clean test stop start release prepare-relase

BIN_DIR   := bin
NAME      := mpv_webrpc
BINARY    := $(BIN_DIR)/$(NAME)
SOURCES   := $(shell find src -name '*.rs')
TEMPLATES := $(shell find templates -name '*.html' -type f -newer $(BINARY))
ELECTRON  := osd/node_modules/.bin/electron

#############################################################################

all: build

build: $(BINARY) $(ELECTRON)

release: prepare-relase dist/$(BINARY) $(ELECTRON)
	cp -r templates/ osd/ settings/ dist/
	cp $(BINARY) bin/osd.sh dist/bin
	cp startup.sh dist/
	mkdir -p dist/db

prepare-relase:
	- rm -rf dist/

run: stop build start

start:
	MEDIAMATE_SETTINGS=settings/settings2.json MEDIAMATE_DB=db/restmpv.db $(BINARY)

stop:
	- killall $(NAME)

clean:
	@rm -f $(BINARY) $(ELECTRON)

$(BINARY): target/debug/$(NAME) $(TEMPLATES)
	- @[ -n "$(TEMPLATES)" ] && echo "Templates changed: $(TEMPLATES)"
	@mkdir -p $(BIN_DIR)
	@cp -v $< $@

dist/$(BINARY): $(BINARY)
	@mkdir -p dist/$(BIN_DIR)
	@cp -v $< $@

target/debug/$(NAME): $(SOURCES)
	cargo --offline build

templates/index.html:
	echo "Updating template: $@

$(ELECTRON):
	npm install electron --prefix osd
	npm audit fix --prefix osd

test:
	runTest.sh
