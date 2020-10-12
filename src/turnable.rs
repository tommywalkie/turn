use doc_comment::doc_comment;
use duplicate::duplicate;

/// **WIP** Rust example Markdown generator macro for internal usage
macro_rules! _example {
    ($e:literal) => {
        concat!("```rust\n", $e, "\n```")
    };
}

/// **WIP** Rust example stringified code generator macro for internal usage
fn _example_code(input_type: String, input: String, res: String) -> String {
    format!("let x: {} = {};\nprintln(x); // {}", input_type, input, res)
}

/// Trait denoting a certain type which can be turned into _something else_
pub trait Turnable {
    doc_comment!("Returns a `bool` value from any input", fn bool(&self) -> bool;);
    doc_comment!("Returns a `String` value from any input", fn print(&self) -> String;);
    doc_comment!("Returns a `Vec<u8>` bytes vector from any input", fn to_vec_u8(&self) -> Vec<u8>;);
    doc_comment!("Returns a `Vec<char>` bytes vector from any input", fn to_vec_char(&self) -> Vec<char>;);
}

#[duplicate(
    docu original_type;
    ["Apply `Turnable` trait to the `i8` type"] [ i8 ];
    ["Apply `Turnable` trait to the `i16` type"] [ i16 ];
    ["Apply `Turnable` trait to the `i32` type"] [ i32 ];
    ["Apply `Turnable` trait to the `i64` type"] [ i64 ];
    ["Apply `Turnable` trait to the `i128` type"] [ i128 ];
    ["Apply `Turnable` trait to the `u8` type"] [ u8 ];
    ["Apply `Turnable` trait to the `u16` type"] [ u16 ];
    ["Apply `Turnable` trait to the `u32` type"] [ u32 ];
    ["Apply `Turnable` trait to the `u64` type"] [ u64 ];
    ["Apply `Turnable` trait to the `u128` type"] [ u128 ];
)]
doc_comment!(docu, impl Turnable for original_type {
    doc_comment!(concat!("Returns a `bool` value from a `", stringify!(original_type), "` integer value"),
        fn bool(&self) -> bool {
            *self != 0
        }
    );
    doc_comment!(concat!("Returns a `String` value from a `", stringify!(original_type), "` integer value"),
        fn print(&self) -> String {
            self.to_string()
        }
    );
    doc_comment!(concat!("Returns a `Vec<u8>` bytes vector from a `", stringify!(original_type), "` integer value"),
        fn to_vec_u8(&self) -> Vec<u8> {
            self.print().as_bytes().to_owned()
        }
    );
    doc_comment!(concat!("Returns a `Vec<char>` vector from a `", stringify!(original_type), "` integer value"),
        fn to_vec_char(&self) -> Vec<char> {
            self.print().chars().collect()
        }
    );
});

#[duplicate(
    docu original_type;
    ["Apply `Turnable` trait to the `f32` type"] [ f32 ];
    ["Apply `Turnable` trait to the `f64` type"] [ f64 ];
)]
doc_comment!(docu, impl Turnable for original_type {
    doc_comment!(concat!("Returns a `bool` value from a `", stringify!(original_type), "` float value"), 
        fn bool(&self) -> bool {
            if self.is_nan() {
                false
            } else {
                *self != 0.0
            }
        }
    );
    doc_comment!(concat!("Returns a `String` value from a `", stringify!(original_type), "` float value"), 
        fn print(&self) -> String {
            format!("{:e}", self)
        }
    );
    doc_comment!(concat!("Returns a `Vec<u8>` bytes vector from a `", stringify!(original_type), "` float value"), 
        fn to_vec_u8(&self) -> Vec<u8> {
            self.print().as_bytes().to_owned()
        }
    );
    doc_comment!(concat!("Returns a `Vec<char>` vector from a `", stringify!(original_type), "` float value"),
        fn to_vec_char(&self) -> Vec<char> {
            self.print().chars().collect()
        }
    );
});

impl Turnable for String {
    doc_comment!(concat!("Returns a `bool` value from a `String` value"),
        fn bool(&self) -> bool {
            self.is_empty()
        }
    );
    doc_comment!(concat!("Returns a `String` value from... a `String` value"), 
        fn print(&self) -> String {
            format!("{}", *self)
        }
    );
    doc_comment!(concat!("Returns a `Vec<u8>` bytes vector from a `String` value"), 
        fn to_vec_u8(&self) -> Vec<u8> {
            self.as_bytes().to_owned()
        }
    );
    doc_comment!(concat!("Returns a `Vec<char>` vector from a `String` value"), 
        fn to_vec_char(&self) -> Vec<char> {
            self.chars().collect()
        }
    );
}
