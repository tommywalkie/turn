#[macro_use]
extern crate doc_comment;
use duplicate::{duplicate_inline};

/// Trait denoting a vector `Vec<T>` which can be turned into `Vec<U>`
pub trait TurnableVec {
    duplicate_inline!{
        [
            to_some_format  return_type;
            [ to_veci8 ]    [ i8 ];
            [ to_veci16 ]    [ i16 ];
            [ to_veci32 ]    [ i32 ];
            [ to_veci64 ]    [ i64 ];
            [ to_veci128 ]    [ i128 ];
            [ to_vecu8 ]    [ u8 ];
            [ to_vecu16 ]    [ u16 ];
            [ to_vecu32 ]    [ u32 ];
            [ to_vecu64 ]    [ u64 ];
            [ to_vecu128 ]    [ u128 ];
            [ to_vecf32 ]    [ f32 ];
            [ to_vecf64 ]    [ f64 ];
        ]
        fn to_some_format(&self) -> Vec<return_type>;
    }
}

duplicate_inline!{
    [
        original_type;
        [ i8 ];
        [ i16 ];
        [ i32 ];
        [ i64 ];
        [ i128 ];
        [ u8 ];
        [ u16 ];
        [ u32 ];
        [ u64 ];
        [ u128 ];
        [ f32 ];
        [ f64 ];
    ]
    /// Apply `TurnableVec` trait to some numeric format
    impl TurnableVec for Vec<original_type> {
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
    }
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
