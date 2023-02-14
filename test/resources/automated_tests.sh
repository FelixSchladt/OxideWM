function setup_check() {
    state=`./target/debug/oxide-msg -c state`

    if grep -q "thread 'main' panicked" <<< $state; then
        echo -e "\x1b[31m\x1b[1mCRITICAL FAILURE\x1b[0m - 'oxide-msg -c state' could not get oxide state - Unable to run tests, aborting..."
        exit
    else
        echo -e "\x1b[32m\x1b[1mSetup Success (1/2)\x1b[0m - Can grab state from OxideWM"
    fi

    if which xterm 1>/dev/null; then
        echo -e "\x1b[32m\x1b[1mSetup Success (2/2)\x1b[0m - 'xterm' is installed."
    else
        echo -e "\x1b[31m\x1b[1mCRITICAL FAILURE\x1b[0m - 'xterm' not found - Unable to run tests, aborting..."
        exit
    fi
}

function run_test() {
    cmd=$1
    success_requirement=$2
    success_message=$3
    failure_message=$4
    counter=$5

    echo -ne "($counter s) Testing: '$cmd'\n\x1b[A"
    bash -c "$cmd"

    while [ $counter -gt 0 ]; do
        sleep 1
        counter=$(( $counter -1 ))
        echo -ne "($counter s) Testing: '$cmd'\n\x1b[A"
    done

    state=`./target/debug/oxide-msg -c state 2>&1`
    if grep -q -E $success_requirement <<< $state; then
        echo -e "\x1b[32m\x1b[1mTEST SUCCESS\x1b[0m - '$cmd' - $success_message"
    else
        echo -e "\x1b[31m\x1b[1mTEST FAILED\x1b[0m  - '$cmd' - $failure_message\n$state"
        echo -e "Note that this failure might influence further tests."
    fi
}

echo -e "\x1b[A\x1b[KSetting up tests...\x1b[0m"
sleep 5 # Sleep required as oxide needs a few seconds to set up it's ipc channel
setup_check

echo -e "\nTesting..."

# Command - Success requirement - Success message - Failure message - Sleep duration
oxidemsg=./target/debug/oxide-msg
run_test "$oxidemsg -c exec --args xterm" "xterm" "Successfully opened a window" "Failed to open a window" 7
run_test "$oxidemsg -c exec --args xterm" "(xterm.*){2}" "Successfully opened a second window" "Failed to open a second window" 7
run_test "$oxidemsg -c layout --args vertical" "[vV]ertical" "Successfully set layout to 'VerticalStriped'" "Failed to set layout to 'VerticalStriped'" 1
run_test "$oxidemsg -c layout --args horizontal" "[hH]orizontal" "Successfully set layout to 'HorizontalStriped'" "Failed to set layout to 'HorizontalStriped'" 1

run_test "$oxidemsg -c kill" "(xterm.*){1}" "Successfully closed a window" "Failed to close a window" 1
run_test "$oxidemsg -c quit" "MethodError" "Successfully quit oxide" "Failed to quit oxide" 2

exit
