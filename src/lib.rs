use duplicate::duplicate_inline;

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
        some_format;
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
    impl TurnableVec for Vec<some_format> {
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
            fn to_some_format(&self) -> Vec<return_type> {
                let mut floats: Vec<return_type> = vec![];
                for i in 0..self.len() {
                    floats.push(self[i] as return_type)
                }
                floats
            }
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
