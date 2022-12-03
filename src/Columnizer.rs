//use num_traits::{Num,Float};
use num_traits::{Float,Int};

/// Maps values to raw column numbers (which can be out of bounds).
pub trait Columnizer<Value> {
    fn get_raw_col(self: &Self, at: Value) -> isize;
}


pub struct RoundingColumnizer<Value> {
    lo: Value,
    col_factor: Value,
}

impl<Value: Float> RoundingColumnizer<Value> {
    pub fn new(lo: Value, hi: Value, max_col: usize) -> RoundingColumnizer<Value> {
        assert!(max_col > 0);
        let total_width = hi - lo;
        let w = (total_width / (max_col as Value)) * 0.5;
        let col_factor = 0.5 / w;
        RoundingColumnizer { lo: lo - w, col_factor, max_col }
    }
}
impl<Value: Float> Columnizer<Value> for RoundingColumnizer<Value>
    where i32: std::convert::From<Value> {
    /// Gets the raw integer column number corresponding to the specified value.
    /// Note that this column number can be negative or > max_col; you have to check.
    fn get_raw_col(self: &Self, at: Value) -> isize {
        i32::from((at - self.lo) * self.col_factor) as isize
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    ///// col_fn tests

    // Test a 1-1 mapping from x to column number.
    #[test]

    fn col_fn1() {
        let mut buf: [u8; 11] = [b'.'; 11];
        let sc = AsciiStripChart::new(&mut buf, 0.0, 10.0);
        assert_eq!(sc.get_raw_col( 0.0 ),  0);
        assert_eq!(sc.get_raw_col(10.0 ), 10);
        assert_eq!(sc.get_raw_col( 5.0 ),  5);
        assert_eq!(sc.get_raw_col( 4.51),  5);
        assert_eq!(sc.get_raw_col( 5.49),  5);
        assert_eq!(sc.get_raw_col( 5.5 ),  6);
    }

    // Test a skewed 1-1 mapping from x to column number.
    #[test]
    fn col_fn2() {
        let mut buf: [u8; 11] = [b'.'; 11];
        let sc = AsciiStripChart::new(&mut buf, -2.0, 8.0);
        assert_eq!(sc.get_raw_col(-2.0 ),  0);
        assert_eq!(sc.get_raw_col( 8.0 ), 10);
        assert_eq!(sc.get_raw_col( 3.0 ),  5);
        assert_eq!(sc.get_raw_col( 2.51),  5);
        assert_eq!(sc.get_raw_col( 3.49),  5);
        assert_eq!(sc.get_raw_col( 3.5 ),  6);
    }

    // Test a skewed 10-1 mapping from x to column number.
    #[test]
    fn col_fn3() {
        let mut buf: [u8; 11] = [b'.'; 11];
        let sc = AsciiStripChart::new(&mut buf, -20.0, 80.0);
        assert_eq!(sc.get_raw_col(-20.0 ),  0);
        assert_eq!(sc.get_raw_col( 80.0 ), 10);
        assert_eq!(sc.get_raw_col( 30.0 ),  5);
        assert_eq!(sc.get_raw_col( 25.1 ),  5);
        assert_eq!(sc.get_raw_col( 34.9 ),  5);
        assert_eq!(sc.get_raw_col( 35.0 ),  6);
    }

    ///// col_fn tests

    //
    #[test]
    fn set1() {
        let mut buf: [u8; 11] = [b'.'; 11];
        let mut sc = AsciiStripChart::new(&mut buf, -20.0, 80.0);
        fn show(sc: &AsciiStripChart) {
            println!("<{}>", unsafe { str::from_utf8_unchecked(sc.buf) });
        }
        show(&sc);
        sc.set(b'|', 0.0);
        show(&sc);
        sc.with(b'A', -9.0, |sc|
            sc.with(b'B', 23.0, |sc| show(&sc))
        );
        show(&sc);
    }

    //
    #[test]
    fn junk() {
        let mut s: [u8; 11] = [b'.'; 11];
        println!("{}", "foo".repeat(5));
        println!("<{:?}>", s);
        println!("<{}>", str::from_utf8(&s).unwrap());
        for i in 0..255 {
            s[0] = i as u8;
            println!("<{}>", unsafe { str::from_utf8_unchecked(&s) });
        }
    }

}
