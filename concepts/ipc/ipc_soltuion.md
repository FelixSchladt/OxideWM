# IPC Solution

Since there are two types of events that have to be handeled, there
needs to be some separation bewteen them.

One type are `xevents`, received from the `X11` instance, and the other one
is custom events create by the user, recieved over `zbus`.

For this reason, each type of event will get its own loop on its own thread,
which will await them and push them into a list shared between them.
The events in this list will be taken care of by the window manager,
who will execute the correct action based on event type and content.

![Solution](solution.png)
