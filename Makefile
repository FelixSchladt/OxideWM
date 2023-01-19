run:
	@echo -e  "\x1b[1m\x1b[36m#- Thank you for using OxideWM -#\x1b[0m"
	./resources/install_required_apps.sh
	./resources/run_oxide.sh

install:
	@echo -e  "\x1b[1m\x1b[36m#- Thank you for installing OxideWM -#\x1b[0m"
	./resources/install_required_apps.sh
	cargo build --release
	cargo build -p oxide-bar --release
	sudo install -Dm755 target/release/oxidewm $(DESTDIR)$(PREFIX)/usr/bin/oxide
	sudo install -Dm755 target/release/oxide-bar $(DESTDIR)$(PREFIX)/usr/bin/oxide-bar
	sudo install -Dm644 resources/oxide.desktop $(DESTDIR)$(PREFIX)/usr/share/xsessions/oxide.desktop
	@echo -e  "\x1b[1m\x1b[36m#- Oxide has been successfully installed -#\x1b[0m"
	@echo -e  "\x1b[1m\x1b[33m#- You can now log out and choose Oxide as you windowmanager -#\x1b[0m"

uninstall:
	@echo -e  "\x1b[1m\x1b[36m#- Uninstalling OxideWM -#\x1b[0m"
	sudo rm -f $(DESTDIR)$(PREFIX)/usr/bin/oxide
	sudo rm -f $(DESTDIR)$(PREFIX)/usr/bin/oxide-bar
	sudo rm -f $(DESTDIR)$(PREFIX)/usr/share/xsessions/oxide.desktop
	@echo -e  "\x1b[1m\x1b[36m#- Oxide has been successfully uninstalled -#\x1b[0m"

