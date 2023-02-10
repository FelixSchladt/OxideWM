function run_test() {
    # $1 = Command
    # $2 = Success requirement
    # $3 = Success message
    # $4 = Failure message
    # $5 = Sleep duration, use when a oxide action takes some time to complete

    counter=$5

    echo -e "($5 s) Testing: '$1'   \t\x1b[A\x1b[K"
    bash -c "$1"

    while [ $counter -gt 0 ]; do
        sleep 1
        counter=$(( $counter -1 ))
        echo -e "($counter s) Testing: '$1'  \x1b[A\x1b[K"
    done

    state=`./target/debug/oxide-msg -c state`
    if grep -q -E $2 <<< $state; then
        echo -e "\x1b[32m\x1b[1mTEST SUCCESS\x1b[0m - '$1' - $3"
    else
        echo -e "\x1b[31m\x1b[1mTEST FAILED\x1b[0m  - '$1' - $4\n$state"
    fi
}

echo -e "\x1b[A\x1b[KSetting up tests...\x1b[0m"

# Sleep required as oxide needs a few seconds to set up it's ipc channel
sleep 5

oxidemsg=./target/debug/oxide-msg
state=`$oxidemsg -c state`

if grep -q "thread 'main' panicked" <<< $state; then
    echo -e "\x1b[31m\x1b[1mCRITICAL FAILURE\x1b[0m - 'oxide-msg -c state' could not get oxide state - Unable to run tests, aborting..."
    exit
else
    echo -e "\x1b[32m\x1b[1mSETUP SUCCESS\x1b[0m - Can grab state from OxideWM"
fi

# Command - Success requirement - Success message - Failure message - Sleep duration
run_test "$oxidemsg -c exec --args xterm" "xterm" "Successfully opened a window" "Failed to open a window" 10

exit
