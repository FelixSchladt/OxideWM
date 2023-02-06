ROOT_DIR := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

SHARE_DIR := /usr/share
TARGET_DIR := /usr/bin
CONFIG_DIR := /etc

define gen_manpages
	pandoc --standalone --to man $(ROOT_DIR)/man/src/oxide-msg.1.md -o $(ROOT_DIR)/man/oxide-msg.1
	pandoc --standalone --to man $(ROOT_DIR)/man/src/config.1.md -o $(ROOT_DIR)/man/config.1
endef

run:
	cd $(ROOT_DIR)
	@echo -e  "\x1b[1m\x1b[36m#- Thank you for using OxideWM -#\x1b[0m"
	$(ROOT_DIR)/resources/install_required_apps.sh
	$(ROOT_DIR)/resources/run_oxide.sh

install:
	@echo -e  "\x1b[1m\x1b[36m#- Thank you for installing OxideWM -#\x1b[0m"
	cd $(ROOT_DIR)
	$(ROOT_DIR)/resources/install_required_apps.sh
	cargo build --release
	cargo build -p oxide-bar --release
	cargo build -p oxide-msg --release
	sudo mkdir -p $(CONFIG_DIR)/oxide
	sudo install -Dm755 \
		$(ROOT_DIR)/target/release/oxide \
		$(ROOT_DIR)/target/release/oxide-bar \
		$(ROOT_DIR)/target/release/oxide-msg \
		-t $(TARGET_DIR)
	sudo cp -t $(CONFIG_DIR)/oxide/ \
		$(ROOT_DIR)/resources/config.yml \
		$(ROOT_DIR)/bar_config.yml
	sudo install -Dm644 $(ROOT_DIR)/resources/oxide.desktop $(SHARE_DIR)/xsessions/oxide.desktop
	cd $(ROOT_DIR) && cargo clean
	@echo -e  "\x1b[1m\x1b[36m#- Oxide has been successfully installed -#\x1b[0m"
	@echo -e  "\x1b[1m\x1b[33m#- You can now log out and choose Oxide as you windowmanager -#\x1b[0m"

uninstall:
	@echo -e  "\x1b[1m\x1b[36m#- Uninstalling OxideWM -#\x1b[0m"
	sudo rm -f\
		$(TARGET_DIR)/oxide\
		$(TARGET_DIR)/oxide-bar\
		$(TARGET_DIR)/oxide-msg\
		$(SHARE_DIR)/xsessions/oxide.desktop\
		$(CONFIG_DIR)/oxide/config.yml
	@echo -e  "\x1b[1m\x1b[36m#- Oxide has been successfully uninstalled -#\x1b[0m"

.PHONY: test
test:
	$(ROOT_DIR)/test/resources/run_tests.sh

.PHONY: man
man:
	$(call gen_manpages)
