#!/bin/bash
assert() {
  echo "==========================================================================="
  expected="$1"
  input="$2"

  cargo run "$input" > tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 0
assert 42 42
assert 44 42+2
assert 40 42-2 
assert 22 "11 * 2" 
assert 11 "(11 * 2) / 2" 
assert 31 "((11 * 2) / 2) + 20" 
assert 42 "11 - -31" 
assert 20 "51 + -31" 

echo OK