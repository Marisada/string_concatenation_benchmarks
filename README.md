# Comparing ways to concatenate strings in Stable Rust 1.81
## Intro
From [https://github.com/hoodie/concatenation_benchmarks-rs](https://github.com/hoodie/concatenation_benchmarks-rs) using [Criterion.rs](https://github.com/bheisler/criterion.rs), remove tests and `from_bytes` unix only function.

## How to run
```bash
    cargo bench
```

## Result from my Pantium Gold
| name                                           | time                            |
|------------------------------------------------|---------------------------------|
| array_concat                                   | [64.560 ns 65.787 ns 67.708 ns] |
| array_join                                     | [68.004 ns 70.516 ns 73.545 ns] |
| array_join_long                                | [62.776 ns 62.852 ns 62.930 ns] |
| collect_from_array_to_string                   | [133.34 ns 133.51 ns 133.68 ns] |
| collect_from_vec_to_string                     | [177.28 ns 177.54 ns 177.80 ns] |
| format_macro                                   | [224.31 ns 224.61 ns 224.89 ns] |
| format_macro_implicit_args                     | [154.53 ns 154.68 ns 154.82 ns] |
| mut_string_push_str                            | [133.36 ns 133.52 ns 133.68 ns] |
| mut_string_push_string                         | [264.67 ns 265.97 ns 268.06 ns] |
| mut_string_with_capacity_push_str              | [60.848 ns 61.007 ns 61.204 ns] |
| mut_string_with_capacity_push_str_char         | [60.417 ns 60.497 ns 60.594 ns] |
| mut_string_with_too_little_capacity_push_str   | [195.97 ns 196.18 ns 196.39 ns] |
| mut_string_with_too_much_capacity_push_str     | [60.808 ns 61.017 ns 61.313 ns] |
| string_from_all                                | [221.30 ns 221.62 ns 222.03 ns] |
| string_from_plus_op                            | [128.80 ns 128.98 ns 129.17 ns] |
| to_owned_plus_op                               | [129.23 ns 129.43 ns 129.64 ns] |
| to_string_plus_op                              | [129.34 ns 129.81 ns 130.47 ns] |
| concat_in_place_macro                          | [114.26 ns 114.40 ns 114.56 ns] |
| string_concat_macro                            | [61.636 ns 61.764 ns 61.925 ns] |
| concat_strs_macro                              | [61.549 ns 61.631 ns 61.715 ns] |
| concat_string_macro                            | [61.541 ns 61.622 ns 61.703 ns] |
| joinery                                        | [150.08 ns 150.31 ns 150.54 ns] |


### Result sorted
| name                                           | time                            |
|------------------------------------------------|---------------------------------|
| mut_string_with_capacity_push_str_char         | [60.417 ns 60.497 ns 60.594 ns] |
| mut_string_with_capacity_push_str              | [60.848 ns 61.007 ns 61.204 ns] |
| mut_string_with_too_much_capacity_push_str     | [60.808 ns 61.017 ns 61.313 ns] |
| concat_string_macro                            | [61.541 ns 61.622 ns 61.703 ns] |
| concat_strs_macro                              | [61.549 ns 61.631 ns 61.715 ns] |
| string_concat_macro                            | [61.636 ns 61.764 ns 61.925 ns] |
| array_join_long                                | [62.776 ns 62.852 ns 62.930 ns] |
| array_concat                                   | [64.560 ns 65.787 ns 67.708 ns] |
| array_join                                     | [68.004 ns 70.516 ns 73.545 ns] |
| concat_in_place_macro                          | [114.26 ns 114.40 ns 114.56 ns] |
| string_from_plus_op                            | [128.80 ns 128.98 ns 129.17 ns] |
| to_owned_plus_op                               | [129.23 ns 129.43 ns 129.64 ns] |
| to_string_plus_op                              | [129.34 ns 129.81 ns 130.47 ns] |
| collect_from_array_to_string                   | [133.34 ns 133.51 ns 133.68 ns] |
| mut_string_push_str                            | [133.36 ns 133.52 ns 133.68 ns] |
| joinery                                        | [150.08 ns 150.31 ns 150.54 ns] |
| format_macro_implicit_args                     | [154.53 ns 154.68 ns 154.82 ns] |
| collect_from_vec_to_string                     | [177.28 ns 177.54 ns 177.80 ns] |
| mut_string_with_too_little_capacity_push_str   | [195.97 ns 196.18 ns 196.39 ns] |
| string_from_all                                | [221.30 ns 221.62 ns 222.03 ns] |
| format_macro                                   | [224.31 ns 224.61 ns 224.89 ns] |
| mut_string_push_string                         | [264.67 ns 265.97 ns 268.06 ns] |

### Result sorted with source
mut_string_with_capacity_push_str_char          
> [60.417 ns 60.497 ns 60.594 ns]
```rust
    let mut datetime = String::with_capacity(20);
    datetime.push_str(a);
    datetime.push('T');
    datetime.push_str(c);
    datetime
```

mut_string_with_capacity_push_str               
> [60.848 ns 61.007 ns 61.204 ns]
```rust
    let mut datetime = String::with_capacity(20);
    datetime.push_str(a);
    datetime.push_str(b);
    datetime.push_str(c);
    datetime
```

mut_string_with_too_much_capacity_push_str      
> [60.808 ns 61.017 ns 61.313 ns]
```rust
    let mut datetime = String::with_capacity(200);
    datetime.push_str(a);
    datetime.push_str(b);
    datetime.push_str(c);
    datetime
```

concat_string_macro                             
> [61.541 ns 61.622 ns 61.703 ns]
```rust
    concat_string!(a, b, c)
```

concat_strs_macro                               
> [61.549 ns 61.631 ns 61.715 ns]
```rust
    concat_strs!(a, b, c)
```

string_concat_macro                             
> [61.636 ns 61.764 ns 61.925 ns]
```rust
    string_concat::string_concat!(a, b, c)
```

array_join_long                                 
> [62.776 ns 62.852 ns 62.930 ns]
```rust
    [a, b, c].join("")
```

array_concat                                    
> [64.560 ns 65.787 ns 67.708 ns]
```rust
    [a, b, c].concat()
```

array_join                                      
> [68.004 ns 70.516 ns 73.545 ns]
```rust
    [a, c].join(b)
```

concat_in_place_macro                           
> [114.26 ns 114.40 ns 114.56 ns]
```rust
    let mut url = String::new();
    concat_in_place::strcat!(&mut url, a b c).to_string()
```

string_from_plus_op                             
> [128.80 ns 128.98 ns 129.17 ns]
```rust
    a.to_string() + b + c
```
to_owned_plus_op                                
> [129.23 ns 129.43 ns 129.64 ns]
```rust
    a.to_owned() + b + c
```

to_string_plus_op                               
> [129.34 ns 129.81 ns 130.47 ns]
```rust
    String::from(a) + b + c
```

collect_from_array_to_string                    
> [133.34 ns 133.51 ns 133.68 ns]
```rust
    let list = [a, b, c];
    list.iter().copied().collect()
```

mut_string_push_str                             
> [133.36 ns 133.52 ns 133.68 ns]
```rust
    let mut datetime = String::new();
    datetime.push_str(a);
    datetime.push_str(b);
    datetime.push_str(c);
    datetime
```

joinery                                         
> [150.08 ns 150.31 ns 150.54 ns]
```rust
    let vec = [a, b, c];
    vec.iter().join_concat().to_string()
```

format_macro_implicit_args                      
> [154.53 ns 154.68 ns 154.82 ns]
```rust
    format!("{a}{b}{c}")
```

collect_from_vec_to_string                      
> [177.28 ns 177.54 ns 177.80 ns]
```rust
    let list = vec![a, b, c];
    list.iter().copied().collect()
```

mut_string_with_too_little_capacity_push_str    
> [195.97 ns 196.18 ns 196.39 ns]
```rust
    let mut datetime = String::with_capacity(2);
    datetime.push_str(a);
    datetime.push_str(b);
    datetime.push_str(c);
    datetime
```

string_from_all                                 
> [221.30 ns 221.62 ns 222.03 ns]
```rust
    String::from(a) + &String::from(b) + &String::from(c)
```

format_macro                                    
> [224.31 ns 224.61 ns 224.89 ns]
```rust
    format!("{}{}{}", a, b, c)
```

mut_string_push_string                          
> [264.67 ns 265.97 ns 268.06 ns]
```rust
    let mut datetime = Vec::<String>::new();
    datetime.push(String::from(a));
    datetime.push(String::from(b));
    datetime.push(String::from(c));
    datetime.join("")
```
