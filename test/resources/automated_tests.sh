function run_test() {
    # $1 = Command
    # $2 = Success requirement
    # $3 = Success message
    # $4 = Failure message
    # $5 = Sleep duration, use when a oxide action takes some time to complete

    echo -e "Testing: '$1'\x1b[A\x1b[K"
    bash -c "$1"
    sleep $5
    state="$( ./target/debug/oxide-msg -c state )"

    if [[ -z "$(grep '$2' <<< $state)" ]]; then
        echo -e "\x1b[31m\x1b[1mTEST FAILED\x1b[0m  - '$1' - $4\n$state"
    else
        echo -e "\x1b[32m\x1b[1mTEST SUCCESS\x1b[0m - '$1' - $3"
    fi
}

cargo build &>/dev/null && ./target/debug/oxide &>/dev/null &
cargo build -p oxide-msg &>/dev/null

sleep 3

oxidemsg=./target/debug/oxide-msg
state=$( $oxidemsg -c state | grep "panic" )

if [[ -z $state ]]; then
    echo -e "\x1b[32m\x1b[1mSETUP SUCCESS\x1b[0m - Can grab state from OxideWM"
else
    echo -e "\x1b[31m\x1b[1mCRITICAL FAILURE\x1b[0m - 'oxide-msg -c state' could not get oxide state - Unable to run tests, aborting..."
    exit
fi

run_test "./target/debug/oxide-msg -c exec --args xterm" "xterm" "Successfully opened a window" "Failed to open a window" 10

exit
