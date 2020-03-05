# Comparing ways to concatenate strings in Rust
## Intro

There are many ways to turn a `&str` into a `String` in Rust and therefore many ways to concatenate two `&str`s.

Here I benchmark several different ways to concatenate the strings `"2014-11-28"`, `"T"` and `"12:00:09Z"` into `"2014-11-28T12:00:09Z"`.

Thanks to all the comments on and discussion on [reddit](https://www.reddit.com/r/rust/comments/48fmta/seven_ways_to_concatenate_strings_in_rust_the/) where I posted these originally only 7 benchmarks. Some go into the details of what is going on in the background of these operations.


## How to run?

* benchmarks: `cargo bench`
* tests: `cargo test --benches`


## Results (on my machine)

```bash
$ cargo bench

running 34 tests
test array_concat_test ... ignored
test array_join_long_test ... ignored
test array_join_test ... ignored
test collect_from_array_to_string_test ... ignored
test collect_from_vec_to_string_test ... ignored
test format_macro_test ... ignored
test from_bytes_test ... ignored
test mut_string_push_str_test ... ignored
test mut_string_push_string_test ... ignored
test mut_string_with_capacity_push_str_char_test ... ignored
test mut_string_with_capacity_push_str_test ... ignored
test mut_string_with_too_little_capacity_push_str_test ... ignored
test mut_string_with_too_much_capacity_push_str_test ... ignored
test string_from_all_test ... ignored
test string_from_plus_op_test ... ignored
test to_owned_plus_op_test ... ignored
test to_string_plus_op_test ... ignored
test array_concat                                 ... bench:          38 ns/iter (+/- 27)
test array_join                                   ... bench:          38 ns/iter (+/- 22)
test array_join_long                              ... bench:          35 ns/iter (+/- 18)
test collect_from_array_to_string                 ... bench:          80 ns/iter (+/- 17)
test collect_from_vec_to_string                   ... bench:          76 ns/iter (+/- 27)
test format_macro                                 ... bench:         111 ns/iter (+/- 69)
test from_bytes                                   ... bench:           1 ns/iter (+/- 0)
test mut_string_push_str                          ... bench:          75 ns/iter (+/- 35)
test mut_string_push_string                       ... bench:         164 ns/iter (+/- 77)
test mut_string_with_capacity_push_str            ... bench:          32 ns/iter (+/- 14)
test mut_string_with_capacity_push_str_char       ... bench:          29 ns/iter (+/- 12)
test mut_string_with_too_little_capacity_push_str ... bench:          96 ns/iter (+/- 11)
test mut_string_with_too_much_capacity_push_str   ... bench:          37 ns/iter (+/- 8)
test string_from_all                              ... bench:         133 ns/iter (+/- 101)
test string_from_plus_op                          ... bench:          76 ns/iter (+/- 9)
test to_owned_plus_op                             ... bench:          81 ns/iter (+/- 15)
test to_string_plus_op                            ... bench:          83 ns/iter (+/- 17)

test result: ok. 0 passed; 0 failed; 17 ignored; 17 measured; 0 filtered out
```

## Examples explained


### `array_concat()`
```rust
let datetime = &[DATE, T, TIME].concat();
```


### `array_join()`
```rust
let datetime = &[DATE, TIME].join(T);
```


### `array_join_long()`
```rust
let datetime = &[DATE, T, TIME].join("");
```


### `collect_from_array_to_string()`
```rust
let list = [DATE, T, TIME];
let datetime: String = list.iter().map(|x| *x).collect();
```

### `collect_from_vec_to_string()`
```rust
let list = vec![DATE, T, TIME];
let datetime: String = list.iter().map(|x| *x).collect();
```

### `format_macro()`

```rust
let datetime = &format!("{}{}{}", DATE, T, TIME);
```

### `from_bytes() // ⚠️ don't actually do this`

```rust
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::slice;

let bytes = unsafe { slice::from_raw_parts(DATE.as_ptr(), 20) };

let datetime = OsStr::from_bytes(bytes);
```

### `mut_string_push_str()`

```rust
let mut datetime = String::new();
datetime.push_str(DATE);
datetime.push_str(T);
datetime.push_str(TIME);
```

### `mut_string_push_string()`

```rust
let mut datetime = Vec::<String>::new();
datetime.push(String::from(DATE));
datetime.push(String::from(T));
datetime.push(String::from(TIME));
let datetime = datetime.join("");
```

### `mut_string_with_capacity_push_str()`

```rust
let mut datetime = String::with_capacity(20);
datetime.push_str(DATE);
datetime.push_str(T);
datetime.push_str(TIME);
```

### `mut_string_with_capacity_push_str_char()`

```rust
let mut datetime = String::with_capacity(20);
datetime.push_str(DATE);
datetime.push('T');
datetime.push_str(TIME);
```

### `mut_string_with_too_little_capacity_push_str()`

```rust
let mut datetime = String::with_capacity(2);
datetime.push_str(DATE);
datetime.push_str(T);
datetime.push_str(TIME);
```

### `mut_string_with_too_much_capacity_push_str()`

```rust
let mut datetime = String::with_capacity(200);
datetime.push_str(DATE);
datetime.push_str(T);
datetime.push_str(TIME);
```

### `string_from_all()`

```rust
let datetime = &(String::from(DATE) + &String::from(T) + &String::from(TIME));
```

### `string_from_plus_op()`

```rust
let datetime = &(String::from(DATE) + T + TIME);
```

### `to_owned_plus_op()`

```rust
let datetime = &(DATE.to_owned() + T + TIME);
```

### `to_string_plus_op()`

```rust
let datetime = &(DATE.to_string() + T + TIME);
```

