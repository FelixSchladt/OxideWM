# DBus-Interface Description

OxideWM has a dbus interface for IPC communication. This is primarily used in the `oxide-ipc` library.
This interface mainly gives access to the current state of oxide. This state includes the loaded config, current windows, layouts, workspaces...
It also allows to execute oxide commands.

## Interface

`org.oxide.interface`

## DBUS Method Calls

`get_state()` -> String
Returns the current `OxideState` as a JSON object.

`sent_event(WmActionEvent) -> void`
Executes the given command


## DBUS Signal

`state_change -> String`
returns the current oxide state when change occurs to the subscribers

