#!/bin/bash
assert() {
  echo "==========================================================================="
  expected="$1"
  input="$2"
  is_write="$3"

  cargo run "$input" > tmp.s
  cc -no-pie -o tmp tmp.s
  result=$(./tmp)
  actual="$?"

  if [ "$is_write" == "true" ]; then
    if [ "$result" = "$expected" ]; then
      echo "$input => $result"
    else
      echo "$input => $expected expected, but got $result"
      exit 1
    fi
  elif [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}
assert 0 "main() { return 0; }"
assert 42 "main() { return 42; }"
assert 44 "main() { return 42+2; }"
assert 40 "main() { return 42-2; }"
assert 22 "main() { return 11 * 2; }" 
assert 11 "main() { return (11 * 2) / 2; }" 
assert 31 "main() { return ((11 * 2) / 2) + 20; }" 
assert 42 "main() { return 11 - -31; }" 
assert 20 "main() { return 51 + -31; }" 
assert 0 "main() { return 1 > 1; }"
assert 1 "main() { return 1 >= 1; }"
assert 0 "main() { return 1 < 1; }"
assert 1 "main() { return 1 <= 1; }"
assert 1 "main() { return 1 == 1; }"
assert 0 "main() { return 1 != 1; }"
assert 5 "main() { a = 5; return a; }"
assert 20 "main() {
aa = 5;
bbb = 15;
return aa + bbb;
}"
assert 20 "main() {
aa111 = 5;
bbb = 15;
return aa111 + bbb;
}"

assert 5 "main() {
a = 2;
b = 3;
return a + b;
}"

assert 10 "main() {
a = 10;
if (a == 10) return a;
return 0;
}"

assert 10 "main() {
a = 10;
b = 2;
if (a == 10) return a;
else return b;
}"
assert 2 "main() {
a = 10;
b = 2;
if (a != 10) return a;
else return b;
}"
assert 10 "main() {
a = 0;
while (a != 10) a = a + 1;
return a;
}"
assert 55 "main() {
foo = 0;
bar = 0;
for (i = 0; i < 10; i = i + 1) {
  bar = bar + 1;
  foo = foo + bar;
}
return foo;
}"

assert 15 "main() {
foo = 0;
while (foo <= 10) {
  foo = foo + 1;
  if (foo == 5) {
    foo = 15;
  }
}
return foo;
}"
assert 48 "
main() {
  print(48);
}
" true
assert 100 "
test() {
  print(100);
}
main() {
  test();
  return 0;
}
" true
assert 0123456789 "
main() {
  for (i = 0; i < 10; i = i + 1) {
    print(i);
  }
}
" true
echo OK