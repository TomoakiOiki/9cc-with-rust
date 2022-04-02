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
assert 2 "10/(1+4)"
assert 47 '5+6*7'
assert 15 '5*(9-6)'
assert 4 '(3+5)/2'
assert 10 '-10+20'
assert 20 '10--10'
assert 0 '-10-(-10)'

echo OK
