#! /bin/sh

if [ -z "$1" ]; then
    echo "Missing: path to rtc binary"
    exit 1
fi

if [ -z "$2" ]; then
    echo "Missing: rtc arguments (surrounded by ' ')"
    exit 1
fi

rm -f out.stacks
sudo dtrace -c "${1} ${2}" -o out.stacks -n 'profile-997 /execname == "rtc"/ { @[ustack(100)] = count(); }'

stackcollapse.pl out.stacks | flamegraph.pl > cpu.svg
