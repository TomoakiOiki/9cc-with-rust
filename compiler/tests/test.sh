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
assert 25 "10 + 5*3"
assert 10 "10/(1+4)"

echo OK
