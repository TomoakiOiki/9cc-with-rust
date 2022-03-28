#!bin/bash
assert(){
    expect="$1"
    input="$2"

    ../target/debug/compiler "$input" > tmp.s
    cc -o tmp tmp.s
    ./tmp
    actual="$?"

    if [ "$actual" = "$expect" ]; then
        echo "$input => $actual"
    else
        echo "$input => $actual (expect $expect)"
        exit 1
    fi
}

assert 27 "5 + 23 - 1"
assert 51 "10 + 44-3"

echo OK
