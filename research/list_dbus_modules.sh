dbus-daemon --introspect | grep -E '<interface name="' | sed -e 's/  <interface name="/Interface: /g' -e 's/">//g'
