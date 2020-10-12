use crate::turnable::*;
use doc_comment::doc_comment;
use duplicate::duplicate;

/// Trait denoting a vector `Vec<T>` which can be turned into `Vec<U>`
pub trait TurnableVec {
    #[duplicate(
        doc to_some_format return_type;
        ["Returns a vector with `i8` values"] [ to_vec_i8 ] [ i8 ];
        ["Returns a vector with `i16` values"] [ to_vec_i16 ] [ i16 ];
        ["Returns a vector with `i32` values"] [ to_vec_i32 ] [ i32 ];
        ["Returns a vector with `i64` values"] [ to_vec_i64 ] [ i64 ];
        ["Returns a vector with `i128` values"] [ to_vec_i128 ] [ i128 ];
        ["Returns a vector with `u8` values"] [ to_vec_u8 ] [ u8 ];
        ["Returns a vector with `u16` values"] [ to_vec_u16 ] [ u16 ];
        ["Returns a vector with `u32` values"] [ to_vec_u32 ] [ u32 ];
        ["Returns a vector with `u64` values"] [ to_vec_u64 ] [ u64 ];
        ["Returns a vector with `u128` values"] [ to_vec_u128 ] [ u128 ];
        ["Returns a vector with `f32` values"] [ to_vec_f32 ] [ f32 ];
        ["Returns a vector with `f64` values"] [ to_vec_f64 ] [ f64 ];
    )]
    doc_comment!(concat!(doc, " from a certain `Vec<T>` input"), 
        fn to_some_format(&self) -> Vec<return_type>;
    );
    doc_comment!(concat!("Returns a vector with `String` values from a certain `Vec<T>` input"), 
        fn to_vec_string(&self) -> Vec<String>;
    );
}

#[duplicate(
    docu original_type;
    ["Apply `TurnableVec` trait to a `Vec<i8>` vector"] [ i8 ];
    ["Apply `TurnableVec` trait to a `Vec<i16>` vector"] [ i16 ];
    ["Apply `TurnableVec` trait to a `Vec<i32>` vector"] [ i32 ];
    ["Apply `TurnableVec` trait to a `Vec<i64>` vector"] [ i64 ];
    ["Apply `TurnableVec` trait to a `Vec<i128>` vector"] [ i128 ];
    ["Apply `TurnableVec` trait to a `Vec<u8>` vector"] [ u8 ];
    ["Apply `TurnableVec` trait to a `Vec<u16>` vector"] [ u16 ];
    ["Apply `TurnableVec` trait to a `Vec<u32>` vector"] [ u32 ];
    ["Apply `TurnableVec` trait to a `Vec<u64>` vector"] [ u64 ];
    ["Apply `TurnableVec` trait to a `Vec<u128>` vector"] [ u128 ];
)]
doc_comment!(docu, impl TurnableVec for Vec<original_type> {
    #[duplicate(
        doc to_some_format return_type;
        ["Returns a vector with `i8` values"] [ to_vec_i8 ] [ i8 ];
        ["Returns a vector with `i16` values"] [ to_vec_i16 ] [ i16 ];
        ["Returns a vector with `i32` values"] [ to_vec_i32 ] [ i32 ];
        ["Returns a vector with `i64` values"] [ to_vec_i64 ] [ i64 ];
        ["Returns a vector with `i128` values"] [ to_vec_i128 ] [ i128 ];
        ["Returns a vector with `u8` values"] [ to_vec_u8 ] [ u8 ];
        ["Returns a vector with `u16` values"] [ to_vec_u16 ] [ u16 ];
        ["Returns a vector with `u32` values"] [ to_vec_u32 ] [ u32 ];
        ["Returns a vector with `u64` values"] [ to_vec_u64 ] [ u64 ];
        ["Returns a vector with `u128` values"] [ to_vec_u128 ] [ u128 ];
        ["Returns a vector with `f32` values"] [ to_vec_f32 ] [ f32 ];
        ["Returns a vector with `f64` values"] [ to_vec_f64 ] [ f64 ];
    )]
    doc_comment!(concat!(doc, " from a `Vec<", stringify!(original_type), ">`"), 
        fn to_some_format(&self) -> Vec<return_type> {
            let mut res: Vec<return_type> = vec![];
            for i in 0..self.len() {
                res.push(self[i] as return_type)
            }
            res
        }
    );
    doc_comment!(concat!("Returns a vector with `String` values from a `Vec<", stringify!(original_type), ">`"),
        fn to_vec_string(&self) -> Vec<String> {
            let mut res: Vec<String> = vec![];
            for i in 0..self.len() {
                res.push(self[i].print())
            }
            res
        }
    );
});

#[duplicate(
    docu original_type;
    ["Apply `TurnableVec` trait to a `Vec<f32>` vector"] [ f32 ];
    ["Apply `TurnableVec` trait to a `Vec<f64>` vector"] [ f64 ];
)]
doc_comment!(docu, impl TurnableVec for Vec<original_type> {
    #[duplicate(
        doc to_some_format return_type;
        ["Returns a vector with `i8` values"] [ to_vec_i8 ] [ i8 ];
        ["Returns a vector with `i16` values"] [ to_vec_i16 ] [ i16 ];
        ["Returns a vector with `i32` values"] [ to_vec_i32 ] [ i32 ];
        ["Returns a vector with `i64` values"] [ to_vec_i64 ] [ i64 ];
        ["Returns a vector with `i128` values"] [ to_vec_i128 ] [ i128 ];
        ["Returns a vector with `u8` values"] [ to_vec_u8 ] [ u8 ];
        ["Returns a vector with `u16` values"] [ to_vec_u16 ] [ u16 ];
        ["Returns a vector with `u32` values"] [ to_vec_u32 ] [ u32 ];
        ["Returns a vector with `u64` values"] [ to_vec_u64 ] [ u64 ];
        ["Returns a vector with `u128` values"] [ to_vec_u128 ] [ u128 ];
        ["Returns a vector with `f32` values"] [ to_vec_f32 ] [ f32 ];
        ["Returns a vector with `f64` values"] [ to_vec_f64 ] [ f64 ];
    )]
    doc_comment!(concat!(doc, " from a `Vec<", stringify!(original_type), ">`"), fn to_some_format(&self) -> Vec<return_type> {
        let mut res: Vec<return_type> = vec![];
        for i in 0..self.len() {
            res.push(self[i] as return_type)
        }
        res
    });
    doc_comment!(concat!("Returns a vector with `String` values from a `Vec<", stringify!(original_type), ">`"),
        fn to_vec_string(&self) -> Vec<String> {
            let mut res: Vec<String> = vec![];
            for i in 0..self.len() {
                res.push(format!("{:e}", self[i]))
            }
            res
        }
    );
});
