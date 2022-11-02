# IPC Mechanism to be implemented

## Feature List
The following feature list is still in an early discussion state and must be seen as an proposal.
The following is written from a highlevel perspective. The transmitted message over dbus shall have a better serializable format. 
The proposed message layout will rather be implemented in an ipc middleware comparable to i3-msg.

**[DISCUSSION]**
* exit - ends window manager session
* restart - restart the window manager
* move $app_id $workspace_id - move an application to workspace
* launch $programm ($workspace_id) - launch an application on an workspace (if none specified default to currently focused)
* get_pids ($workspace_id|focused) - returns currently active apps (none -> all | focused -> on focused workspace | $workspace_id -> on the specified workspace)
    - should this return a datastructure containing all window attributes? 
* kill ($pid) -> does this offer benefits over normal kill e.g gracefull exit?
* fullscreen ($pid)-> changes app between fullscreen and not fullscreen 

