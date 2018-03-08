#![feature(test)]
#![allow(dead_code)]

extern crate test;

static DATE: &'static str = "2014-11-28";
static T: &'static str = "T";
static TIME: &'static str = "12:00:09Z";
static DATETIME: &'static str = "2014-11-28T12:00:09Z";


use test::Bencher;

#[bench]
fn collect_to_string(b: &mut Bencher) {
    let list = vec![DATE, T, TIME];
    b.iter(|| {
               let datetime: String = list.iter().map(|x| *x).collect();
               test::black_box(datetime);
           });
}

#[bench]
fn format_macro(b: &mut Bencher) {
    b.iter(|| {
               let datetime: &str = &format!("{}T{}", DATE, TIME);
               test::black_box(datetime);
           });
}

#[bench]
fn to_string_plus_op(b: &mut Bencher) {
    b.iter(|| {
               let datetime: &str = &(DATE.to_string() + "T" + TIME);
               test::black_box(datetime);
           });
}

#[bench]
fn to_owned_plus_op(b: &mut Bencher) {
    b.iter(|| {
               let datetime: &str = &(DATE.to_owned() + "T" + TIME);
               test::black_box(datetime);
           });
}

#[bench]
fn string_from_plus_op(b: &mut Bencher) {
    b.iter(|| {
               let datetime: &str = &(String::from(DATE) + "T" + TIME);
               test::black_box(datetime);
           });
}

#[bench]
fn string_from_all(b: &mut Bencher) {
    b.iter(|| {
               let datetime: &str = &(String::from(DATE) + &String::from("T") + &String::from(TIME));
               test::black_box(datetime);
           });
}

#[bench]
fn write_macro(b: &mut Bencher) {
    b.iter(|| {
               use std::io::Write;
               let mut datetime = Vec::new();
               write!(&mut datetime, "{}T{}", DATE, TIME).unwrap();
               test::black_box(datetime);
           });
}

#[bench]
fn mut_string_with_capacity_push_str(b: &mut Bencher) {
    b.iter(|| {
               let mut datetime = String::with_capacity(20);
               datetime.push_str(DATE);
               datetime.push_str("T");
               datetime.push_str(TIME);
               test::black_box(datetime);
           });
}

#[bench]
fn mut_string_with_capacity_push_str_char(b: &mut Bencher) {
    b.iter(|| {
               let mut datetime = String::with_capacity(20);
               datetime.push_str(DATE);
               datetime.push('T');
               test::black_box(datetime);
           });
}

#[bench]
fn mut_string_with_too_little_capacity_push_str(b: &mut Bencher) {
    b.iter(|| {
               let mut datetime = String::with_capacity(2);
               datetime.push_str(DATE);
               datetime.push_str("T");
               datetime.push_str(TIME);
               test::black_box(datetime);
           });
}

#[bench]
fn mut_string_with_too_much_capacity_push_str(b: &mut Bencher) {
    b.iter(|| {
               let mut datetime = String::with_capacity(200);
               datetime.push_str(DATE);
               datetime.push_str("T");
               datetime.push_str(TIME);
               test::black_box(datetime);
           });
}

#[bench]
fn mut_string_push_string(b: &mut Bencher) {
    b.iter(|| {
               let mut datetime = Vec::<String>::new();
               datetime.push(String::from(DATE));
               datetime.push(String::from("T"));
               datetime.push(String::from(TIME));
               let datetime = datetime.join("");
               test::black_box(datetime);
           });
}

#[bench]
fn mut_string_push_str(b: &mut Bencher) {
    b.iter(|| {
               let mut datetime = String::new();
               datetime.push_str(DATE);
               datetime.push_str("T");
               datetime.push_str(TIME);
               test::black_box(datetime);
           });
}

#[bench]
fn array_concat(b: &mut Bencher) {
    b.iter(|| {
               let datetime: &str = &[DATE, "T", TIME].concat();
               test::black_box(datetime);
           });
}

#[bench]
fn array_join(b: &mut Bencher) {
    b.iter(|| {
               let datetime: &str = &[DATE, TIME].join("T");
               test::black_box(datetime);
           });
}

#[bench]
fn array_join_empty_arg(b: &mut Bencher) {
    b.iter(|| {
               let datetime: &str = &[DATE, "T", TIME].join("");
               test::black_box(datetime);
           });
}

#[bench]
#[cfg(unix)]
fn from_bytes(b: &mut Bencher) {
    use std::slice;
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    b.iter(|| {
               let bytes = unsafe { slice::from_raw_parts(DATE.as_ptr(), 20) };

               let datetime = OsStr::from_bytes(bytes);
               test::black_box(datetime);
           });
}
