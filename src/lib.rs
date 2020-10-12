mod turnable;
pub use crate::turnable::Turnable;

mod vec;
pub use crate::vec::TurnableVec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a: Vec<u8> = vec![0, 1, 2, 3];
        let b: Vec<f64> = vec![0.1, 1.1, 2.1, 3.1];
        let c: Vec<f64> = vec![0.9, 1.9, 2.9, 3.9];
        let d: u8 = 5;
        let e: f64 = 5.0;
        let f: f64 = 5.825;
        let g: String = String::from("hello");
        let h: String = String::from("h");
        let i: u32 = 2;
        assert_eq!(a.to_vec_f64(), vec![0.0, 1.0, 2.0, 3.0]);
        assert_eq!(b.to_vec_f64(), vec![0.1, 1.1, 2.1, 3.1]);
        assert_eq!(b.to_vec_u8(), vec![0, 1, 2, 3]);
        assert_eq!(c.to_vec_u8(), vec![0, 1, 2, 3]);
        assert_eq!(a.to_vec_string(), vec!["0", "1", "2", "3"]);
        assert_eq!(b.to_vec_string(), vec!["1e-1", "1.1e0", "2.1e0", "3.1e0"]);
        assert_eq!(d.print(), "5");
        assert_eq!(e.print(), "5e0");
        assert_eq!(f.print(), "5.825e0");
        assert_eq!(g.to_vec_u8(), vec![104, 101, 108, 108, 111]);
        assert_eq!(h.to_vec_u8(), vec![104]);
        assert_eq!(i.to_vec_char(), vec!['2']);
    }
}
