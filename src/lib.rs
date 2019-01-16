pub struct AsciiStripChart<'a> {
    lo: f32,
    col_factor: f32,
    max_col: usize,
    buf: &'a mut [u8],
}

impl<'a> AsciiStripChart<'a> {
    /// Returns an AsciiStripChart that maps values within a given range
    /// into the specified buffer.
    pub fn new(buf: &mut [u8], lo: f32, hi: f32) -> AsciiStripChart {
        let max_col = buf.len() - 1;
        assert!(max_col > 0);
        let total_width = hi - lo;
        let w = (total_width / (max_col as f32)) * 0.5;
        let col_factor = 0.5 / w;
        AsciiStripChart { lo: lo - w, col_factor, max_col, buf }
    }
    /// Temporarily puts a mark at the specified f32 location and then
    /// calls the specified function.
    pub fn with(self: &mut Self, mark: u8, at: f32, f: fn(&mut Self) -> ()) {
        let (i, ch) = self.get_col(mark, at);
        let old = self.buf[i];
        self.buf[i] = ch;
        f(self);
        self.buf[i] = old;
    }
    /// Puts a mark at the specified f32 location.
    pub fn set(self: &mut Self, mark: u8, at: f32) -> u8 {
        let (i, ch) = self.get_col(mark, at);
        let old = self.buf[i];
        self.buf[i] = ch;
        old
    }
    /// Given an f32 location and a mark to put there, this function returns
    /// an integer column number in the range 0...max_col and a mark to put there.
    /// The returned mark is the input mark unless the value is too low or too high,
    /// in which cases < or > is returned, respectively.
    pub fn get_col(self: &Self, mark: u8, at: f32) -> (usize, u8) {
        let col = self.get_raw_col(at);
        if col < 0 {
            (0, b'<')
        } else {
            let col = col as usize;
            if col > self.max_col {
                (self.max_col, b'>')
            } else {
                (col, mark)
            }
        }
    }
    /// Gets the raw integer column number corresponding to the specified f32 location.
    /// Note that this column number can be negative or > max_col; you have to check.
    pub fn get_raw_col(self: &Self, at: f32) -> isize {
        ((at - self.lo) * self.col_factor) as isize
    }
    // pub fn col_fn(self: &mut Self) -> impl Fn(f32) -> isize {
    //     let total_width = self.hi - self.lo;
    //     let w = (total_width / (self.max_col as f32)) * 0.5;
    //     let x_lo = self.lo - w;
    //     let col_factor = 0.5 / w;
    //     move |x: f32| -> isize { ((x - x_lo) * col_factor) as isize }
    // }
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
