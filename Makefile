.PHONY: all debug run clean test stop start release

NAME    := mpv_webrpc
SOURCES := $(shell find src -name '*.rs')
ASSETS  := $(shell find templates osd)
CROSS   := $(HOME)/.cargo/bin/cross

MKDIR = @mkdir -p $(@D)
COPY  =	cp $< $@

# x64, arm64
ARCH      := x64
ELECTRON_VERSION := 4.2.12
ELECTRON_ARCHIVE := electron-v$(ELECTRON_VERSION)-linux-$(ARCH).zip
ELECTRON_URL := https://github.com/electron/electron/releases/download/v$(ELECTRON_VERSION)/$(ELECTRON_ARCHIVE)
CACHE_DIR := $(HOME)/.cache

#############################################################################

all: debug

debug: bin/$(NAME) electron

release: dist/$(ARCH)/bin/$(NAME) \
	dist/$(ARCH)/bin/mediamate \
	electron \
	dist/$(ARCH)/bin/osd.sh \
	$(addprefix dist/$(ARCH)/,$(ASSETS))

run: stop build start

start:
	SETTINGS=settings/settings2.json DATABASE_URL=db/restmpv.db bin/$(NAME)

stop:
	- killall $(NAME)

clean:
	rm -rf dist/$(ARCH)

bin/$(NAME): target/debug/$(NAME)
	@cp -v $< $@

#############################################################################

target/debug/$(NAME): $(SOURCES)
	CARGO_BIN_NAME=$(NAME) cargo --offline build

target/x64/release/$(NAME): $(SOURCES)
	CARGO_TARGET_DIR=target/x64 cargo build --release
	- @mv $(@D)/mpv_webrpc $@

target/arm64/release/$(NAME): $(CROSS) aarch64/.image $(SOURCES)
	cd target && mkdir -p aarch64-unknown-linux-gnu && ln -sf aarch64-unknown-linux-gnu arm64
	PATH=$(dir $(CROSS)):$(PATH) $(CROSS) build --release --target=aarch64-unknown-linux-gnu
	- @mv $(@D)/mpv_webrpc $@

#############################################################################

dist/$(ARCH)/bin/$(NAME): target/$(ARCH)/release/$(NAME)
	$(MKDIR)
	$(COPY)

dist/$(ARCH)/bin/osd.sh: bin/osd.sh
	$(MKDIR)
	$(COPY)

dist/$(ARCH)/osd:
	$(MKDIR)
	$(COPY)

#############################################################################

dist/$(ARCH)/bin/mediamate: bin/mediamate
	$(MKDIR)
	$(COPY)

dist/$(ARCH)/osd/%.js: osd/%.js
	$(MKDIR)
	$(COPY)

dist/$(ARCH)/osd/%.html: osd/%.html
	$(MKDIR)
	$(COPY)

dist/$(ARCH)/osd/%.png: osd/%.png
	$(MKDIR)
	$(COPY)

dist/$(ARCH)/templates/%.html: templates/%.html
	$(MKDIR)
	$(COPY)

dist/$(ARCH)/templates/public/%.css: templates/public/%.css
	$(MKDIR)
	$(COPY)

dist/$(ARCH)/templates/public/%.js: templates/public/%.js
	$(MKDIR)
	$(COPY)

dist/$(ARCH)/templates/public/img: templates/public/img/
	$(MKDIR)
	cp -r $< $@

#############################################################################

.PHONY: electron

electron: dist/x64/osd/electron/electron

dist/$(ARCH)/osd/electron/electron: $(CACHE_DIR)/$(ELECTRON_ARCHIVE)
	@echo "Unpacking Electron for $(ARCH)..."
	@mkdir -p $(dir $@)
	@unzip -qo -d $(dir $@) $<
	@cd $(dir $@) && mkdir -p locales2 && \
	mv locales/en-US.pak locales/de.pak locales2 && rm -rf locales && mv locales2 locales
	@touch $@

$(CACHE_DIR)/$(ELECTRON_ARCHIVE):
	@echo "Downloading Electron for $(ARCH)..."
	@mkdir -p $(CACHE_DIR)
	@wget -q -P $(CACHE_DIR) --show-progress $(ELECTRON_URL)
	@touch $(ELECTRON_ARCHIVE)

#############################################################################

$(CROSS):
	cargo install cross

aarch64/.image: aarch64/Dockerfile
	docker build aarch64 --tag mafflerbach/aarch64-unknown-linux-gnu
	touch $@

#############################################################################

test:
	runTest.sh
