use doc_comment::doc_comment;
use duplicate::{duplicate_inline};

/// Trait denoting a vector `Vec<T>` which can be turned into `Vec<U>`
pub trait TurnableVec {
    duplicate_inline!{
        [
            doc to_some_format  return_type;
            ["Returns a vector with `i8` values"] [ to_veci8 ]    [ i8 ];
            ["Returns a vector with `i16` values"] [ to_veci16 ]    [ i16 ];
            ["Returns a vector with `i32` values"] [ to_veci32 ]    [ i32 ];
            ["Returns a vector with `i64` values"] [ to_veci64 ]    [ i64 ];
            ["Returns a vector with `i128` values"] [ to_veci128 ]    [ i128 ];
            ["Returns a vector with `u8` values"] [ to_vecu8 ]    [ u8 ];
            ["Returns a vector with `u16` values"] [ to_vecu16 ]    [ u16 ];
            ["Returns a vector with `u32` values"] [ to_vecu32 ]    [ u32 ];
            ["Returns a vector with `u64` values"] [ to_vecu64 ]    [ u64 ];
            ["Returns a vector with `u128` values"] [ to_vecu128 ]    [ u128 ];
            ["Returns a vector with `f32` values"] [ to_vecf32 ]    [ f32 ];
            ["Returns a vector with `f64` values"] [ to_vecf64 ]    [ f64 ];
        ]
        doc_comment!(doc, fn to_some_format(&self) -> Vec<return_type>;);
    }
}

duplicate_inline!{
    [
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
        ["Apply `TurnableVec` trait to a `Vec<f32>` vector"] [ f32 ];
        ["Apply `TurnableVec` trait to a `Vec<f64>` vector"] [ f64 ];
    ]
    doc_comment!(docu, impl TurnableVec for Vec<original_type> {
        duplicate_inline!{
            [
                doc to_some_format  return_type;
                ["Returns a vector with `i8` values"] [ to_veci8 ]    [ i8 ];
                ["Returns a vector with `i16` values"] [ to_veci16 ]    [ i16 ];
                ["Returns a vector with `i32` values"] [ to_veci32 ]    [ i32 ];
                ["Returns a vector with `i64` values"] [ to_veci64 ]    [ i64 ];
                ["Returns a vector with `i128` values"] [ to_veci128 ]    [ i128 ];
                ["Returns a vector with `u8` values"] [ to_vecu8 ]    [ u8 ];
                ["Returns a vector with `u16` values"] [ to_vecu16 ]    [ u16 ];
                ["Returns a vector with `u32` values"] [ to_vecu32 ]    [ u32 ];
                ["Returns a vector with `u64` values"] [ to_vecu64 ]    [ u64 ];
                ["Returns a vector with `u128` values"] [ to_vecu128 ]    [ u128 ];
                ["Returns a vector with `f32` values"] [ to_vecf32 ]    [ f32 ];
                ["Returns a vector with `f64` values"] [ to_vecf64 ]    [ f64 ];
            ]
            doc_comment!(doc, fn to_some_format(&self) -> Vec<return_type> {
                let mut res: Vec<return_type> = vec![];
                for i in 0..self.len() {
                    res.push(self[i] as return_type)
                }
                res
            });
        }
    });
}

#[cfg(test)]
mod tests {
    use crate::{TurnableVec};

    #[test]
    fn it_works() {
        let a: Vec<u8> = vec![0,1,2,3];
        let b: Vec<f64> = vec![0.1,1.1,2.1,3.1];
        assert_eq!(a.to_vecf64(), vec![0.0,1.0,2.0,3.0]);
        assert_eq!(b.to_vecf64(), vec![0.1,1.1,2.1,3.1]);
    }
}
