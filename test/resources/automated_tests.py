import subprocess
import time
import json
import re

oxide_msg = ''
get_state = ''

def bash(command) -> str:
    command = command.split(' ')
    process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    stdout  = process.communicate()[0].decode('utf-8')

    return stdout.replace('\n', '')

def oxide(command, wait_duration=1) -> str:
    bash(oxide_msg + ' ' + command)
    time.sleep(wait_duration)

    return bash(get_state)


def setup():
    global oxide_msg,\
           get_state

    print("Running Setup...")

    time.sleep(3)

    cwd       = bash('pwd')
    oxide_msg = cwd       + '/target/debug/oxide-msg'
    get_state = oxide_msg + ' state'

    print(f"\x1b[32m\x1b[1mSetup Success (1/4)\x1b[0m - Mapped \'oxide-msg\' to {oxide_msg}")

    if 'MethodError' in bash(get_state):
        print("\x1b[31m\x1b[1mCRITICAL FAILURE\x1b[0m - 'oxide-msg state' could not get oxide state - Unable to run tests, aborting...")
        exit(-1)
    else:
        print("\x1b[32m\x1b[1mSetup Success (2/4)\x1b[0m - Can grab state from OxideWM")

    if '' == bash('which xterm'):
        print("\x1b[31m\x1b[1mCRITICAL FAILURE\x1b[0m - 'xterm' not found - Unable to run tests, aborting...")
        exit(-1)
    else:
        print("\x1b[32m\x1b[1mSetup Success (3/4)\x1b[0m - 'xterm' is installed.")

    if '' == bash('which kitty'):
        print("\x1b[31m\x1b[1mCRITICAL FAILURE\x1b[0m - 'kitty' not found - Unable to run tests, aborting...")
        exit(-1)
    else:
        print("\x1b[32m\x1b[1mSetup Success (4/4)\x1b[0m - 'kitty' is installed.")



def test(function, args=None):
    print(f"Testing \'{function.__name__}\'...")

    result = function(args) if (args != None) else function()

    if result:
        print(f"\x1b[A\x1b[K\x1b[32m\x1b[1mTest Success\x1b[0m - {function.__name__}")
    else:
        print(f"\x1b[A\x1b[K\x1b[31m\x1b[1mTest Failure\x1b[0m - {function.__name__}")


def open_kitty_windows():
    success = True
    states  = [
        oxide('exec kitty'),
        oxide('exec kitty'),
        oxide('exec kitty'),
        oxide('exec kitty'),
    ]

    for index, state in enumerate(states):
        if len(re.findall('kitty', state)) != index + 2:     # One 'kitty' string from the test config, and one because the index starts at 0
            success = False
            break

    return success


def open_xterm_window():
    success = True
    state   = oxide('exec xterm', 7)

    if 'xterm' not in state:
        success = False

    return success


def move_focus():
    get_focused_window = lambda payload : payload['screeninfo']\
                                                 ['1361']\
                                                 ['workspaces']\
                                                 ['1']\
                                                 ['focused_window']

    original_state = json.loads(bash(get_state))
    original_focus = get_focused_window(original_state)

    states  = [
        get_focused_window(json.loads(oxide('focus left'))),
        get_focused_window(json.loads(oxide('focus left'))),
    ]

    return original_state != states[0] != states[1]

def move_window():
    get_window_order = lambda payload : payload['screeninfo']\
                                               ['1361']\
                                               ['workspaces']\
                                               ['1']\
                                               ['order']

    original_state = json.loads(bash(get_state))
    original_order = get_window_order(original_state)

    states  = [
        get_window_order(json.loads(oxide('move left', 2))),
        get_window_order(json.loads(oxide('move left', 2))),
        get_window_order(json.loads(oxide('move left', 2))),
        get_window_order(json.loads(oxide('move left', 2))),
        get_window_order(json.loads(oxide('move right', 2))),
        get_window_order(json.loads(oxide('move right', 2))),
        get_window_order(json.loads(oxide('move right', 2))),
        get_window_order(json.loads(oxide('move right', 2))),
    ]

    for index, state in enumerate(states):
        if index == 0 or index == len(states)-1:
            continue

        if state == states[index-1]:
            return False

    return original_order == states[len(states)-1]

def switch_to_verical_layout():
    state = oxide('layout vertical')
    return 'vertical_striped' in state


def switch_to_horizontal_layout():
    state = oxide('layout horizontal')
    return 'horizontal_striped' in state


def switch_to_tiled_layout():
    state = oxide('layout tiled')
    return 'tiled' in state

def close_window():
    return 'xterm' not in oxide('kill')


def send_invalid_command():
    return "Error: Invalid command! Run 'oxide-msg --help' to view usage." in oxide('invalid command')

def quit_oxide():
    return 'MethodError' in oxide('quit')


def main():
    setup()

    print("#=======================================================================#")
    print("Running Tests...")

    # Stress testing
    test(send_invalid_command)

    # Validate functionality
    test(open_kitty_windows)
    test(open_xterm_window)
    test(move_focus)
    test(move_window)
    test(switch_to_verical_layout)
    test(move_window)
    test(switch_to_horizontal_layout)
    test(move_window)
    test(switch_to_tiled_layout)

    # Test quitting functionality
    test(close_window)
    test(quit_oxide)


if __name__ == "__main__":
    main()
