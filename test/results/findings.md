# Findings

This file describes found issues while testing.
Not all issues listed here might translate into bug reports.

## Application issues

Errors and findings described in this section refer to issues in the application.

### Some applications lead to performance issues

Applications like `gnome-terminal` can perform poorly with the window manager.
Should this happen it is recommended to switch to another program.
The recommended terminal is called `kitty`.

### Keycombinations perform different actions depending on the order they are pressed in

When pressing multiple keys (e.g.: `Alt+T`) the order of the keypresses will influence the performed action.
This is because combined keypresses are combined on a binary level and send from the X server to the window manager as a singluar number.
Since this number changes when the order the keys are pressed in changes, the keycombination is only handeled by the windowmanager when the modifier key (e.g.: `Alt`, `Shift`, `Ctrl`)
is pressed before the letter key.

### All windows flash when a new one is opened or a existing one is closed

The flash happens since everytime a window opens or closes, all windows need to be "remapped", meaning resized and repositioned.
To do this, the X server needs to remove the windows from the screen first and then adds them again with the updated values, resulting in a short flash.

### Multiple screens are not supported

When using the windowmanager with more then one screen, undefined behavior will occur.
It has been observed during manual testing that the manager may treat multiple screens as one combined screen, also it likely depends on
what applications are used to configure and manager the individual screens themselves (e.g. drivers).

### Focus issues
While moving windows or changing focus via the keyboard sometimes unecpected behavior occurs.
This is mostly caused by the mouse not being moved and stays in its position.
Therefore the mouse may be hovering above a not focused window and in the moment a position update (eg slight movement of the mouse etc.) the window under the mouse is focused.

## Testing issues

Errors and findings described in this section refer to issues in the testing framework and not the application itself.

### Config parser error message during integration tests

This error message is wanted behavior and has been implemented to let a user know that they have an issue in their config file.
The windowmanager will automatically load a default configuration in this case to avoid a crash.

### Automated Validation Failure

These warnings during automated testing occur because of variations in the duration the application requires to update it's status.
This is dependant on not easily controllable, project unrelated properties.
Due to this, it may be necessary to validate some functionality manually.
Steps have been taken to minimize manually required testing by running tests prone to failing multiple times.
Should a majority of these tests succeed, the tests can be regarded as passed.

### Automated tests fail when mouse hovers over a window during testing

Since the mouse cursor will set focus, it will move focus to the window that is being hovered over.
Since the tests depend on the correct window being focused, it is advised to move the mouse outside of the testing frame.
This will not crash tests, but only lead to more automatic test validation failures it might be difficult to notice when such a mistakes occurs.

### Automated tests are not possible while mouse is positioned over oxide-bar

Since the window focus follows the mouse cursor, attempting to change focus while the it is positioned above the task-bar will result in a `oxide-ipc` crash.

### Last automated tests may throw "thread main panicked" error

This is wanted behavior, the tests checks whether the windowmanager closed as planned with this very error message.
Due to timing differences outside of the scope of this project, the error can occur at different points in the test method and might be printed.
This has no further effect on any tests.
It is noteworthy that if this error occurs during *any other* test then the very last one, it should be considered a critical failure.
