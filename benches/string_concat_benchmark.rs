#[macro_use]
extern crate string_concat;
#[macro_use(concat_strs)]
extern crate concat_strs;
#[macro_use(concat_string)]
extern crate concat_string;
extern crate joinery;
use joinery::prelude::*;

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};

static DATE: &str = "2014-11-28";
static T: &str = "T";
static TIME: &str = "12:00:09Z";

fn array_concat(a: &str, b: &str, c: &str) -> String {
    [a, b, c].concat()
}

fn array_join(a: &str, b: &str, c: &str) -> String {
    [a, c].join(b)
}

fn array_join_long(a: &str, b: &str, c: &str) -> String {
    [a, b, c].join("")
}

fn collect_from_array_to_string(a: &str, b: &str, c: &str) -> String {
    let list = [a, b, c];
    list.iter().copied().collect()
}

fn collect_from_vec_to_string(a: &str, b: &str, c: &str) -> String {
    let list = vec![a, b, c];
    list.iter().copied().collect()
}

fn format_macro(a: &str, b: &str, c: &str) -> String {
    format!("{}{}{}", a, b, c)
}

/// Implicit named arguments were added in Rust 1.58
fn format_macro_implicit_args(a: &str, b: &str, c: &str) -> String {
    format!("{a}{b}{c}")
}

fn mut_string_push_str(a: &str, b: &str, c: &str) -> String {
    let mut datetime = String::new();
    datetime.push_str(a);
    datetime.push_str(b);
    datetime.push_str(c);
    datetime
}

fn mut_string_push_string(a: &str, b: &str, c: &str) -> String {
    let mut datetime = Vec::<String>::new();
    datetime.push(String::from(a));
    datetime.push(String::from(b));
    datetime.push(String::from(c));
    datetime.join("")
}

fn mut_string_with_capacity_push_str(a: &str, b: &str, c: &str) -> String {
    let mut datetime = String::with_capacity(20);
    datetime.push_str(a);
    datetime.push_str(b);
    datetime.push_str(c);
    datetime
}

fn mut_string_with_capacity_push_str_char(a: &str, _b: &str, c: &str) -> String {
    let mut datetime = String::with_capacity(20);
    datetime.push_str(a);
    datetime.push('T');
    datetime.push_str(c);
    datetime
}

fn mut_string_with_too_little_capacity_push_str(a: &str, b: &str, c: &str) -> String {
    let mut datetime = String::with_capacity(2);
    datetime.push_str(a);
    datetime.push_str(b);
    datetime.push_str(c);
    datetime
}

fn mut_string_with_too_much_capacity_push_str(a: &str, b: &str, c: &str) -> String {
    let mut datetime = String::with_capacity(200);
    datetime.push_str(a);
    datetime.push_str(b);
    datetime.push_str(c);
    datetime
}

fn string_from_all(a: &str, b: &str, c: &str) -> String {
    String::from(a) + &String::from(b) + &String::from(c)
}

fn string_from_plus_op(a: &str, b: &str, c: &str) -> String {
    String::from(a) + b + c
}

fn to_owned_plus_op(a: &str, b: &str, c: &str) -> String {
    a.to_owned() + b + c
}

fn to_string_plus_op(a: &str, b: &str, c: &str) -> String {
    a.to_string() + b + c
}

/// https://crates.io/crates/concat-in-place
fn concat_in_place_macro(a: &str, b: &str, c: &str) -> String {
    let mut url = String::new();
    concat_in_place::strcat!(&mut url, a b c).to_string()
}

/// https://crates.io/crates/string_concat
fn string_concat_macro(a: &str, b: &str, c: &str) -> String {
    string_concat::string_concat!(a, b, c)
}

/// https://crates.io/crates/concat_strs
fn concat_strs_macro(a: &str, b: &str, c: &str) -> String {
    concat_strs!(a, b, c)
}

/// https://crates.io/crates/concat-string
fn concat_string_macro(a: &str, b: &str, c: &str) -> String {
    concat_string!(a, b, c)
}

/// https://crates.io/crates/joinery
fn joinery(a: &str, b: &str, c: &str) -> String {
    let vec = [a, b, c];
    vec.iter().join_concat().to_string()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("array_concat", |b| b.iter(|| array_concat(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("array_join", |b| b.iter(|| array_join(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("array_join_long", |b| b.iter(|| array_join_long(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("collect_from_array_to_string", |b| b.iter(|| collect_from_array_to_string(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("collect_from_vec_to_string", |b| b.iter(|| collect_from_vec_to_string(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("format_macro", |b| b.iter(|| format_macro(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("format_macro_implicit_args", |b| b.iter(|| format_macro_implicit_args(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("mut_string_push_str", |b| b.iter(|| mut_string_push_str(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("mut_string_push_string", |b| b.iter(|| mut_string_push_string(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("mut_string_with_capacity_push_str", |b| b.iter(|| mut_string_with_capacity_push_str(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("mut_string_with_capacity_push_str_char", |b| b.iter(|| mut_string_with_capacity_push_str_char(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("mut_string_with_too_little_capacity_push_str", |b| b.iter(|| mut_string_with_too_little_capacity_push_str(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("mut_string_with_too_much_capacity_push_str", |b| b.iter(|| mut_string_with_too_much_capacity_push_str(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("string_from_all", |b| b.iter(|| string_from_all(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("string_from_plus_op", |b| b.iter(|| string_from_plus_op(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("to_owned_plus_op", |b| b.iter(|| to_owned_plus_op(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("to_string_plus_op", |b| b.iter(|| to_string_plus_op(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("concat_in_place_macro", |b| b.iter(|| concat_in_place_macro(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("string_concat_macro", |b| b.iter(|| string_concat_macro(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("concat_strs_macro", |b| b.iter(|| concat_strs_macro(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("concat_string_macro", |b| b.iter(|| concat_string_macro(black_box(DATE), black_box(T), black_box(TIME))));
    c.bench_function("joinery", |b| b.iter(|| joinery(black_box(DATE), black_box(T), black_box(TIME))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
