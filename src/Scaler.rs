use num_traits::{Float, Num, PrimInt};

// Is this really what I need, or do I need a *factory*
// that allows me to create my *own* Scaler?  The user
// does not know

pub trait Scaler<Value> {
    fn scale(self: &Self, value: Value) -> Value;
}

pub struct IdentityScaler<Value: Num> {}
impl<Value: Num> IdentityScaler<Num> {
    fn new() -> IdentityScaler<Num> {
        IdentityScaler {}
    }
    fn scale(self: &Self, value: Value) -> Value {
        value
    }
}

pub struct FloatScaler<Value: Float> {
    mult: Value,
}
impl<Value: Float> FloatScaler<Value> {
    fn new(mult: Value) -> FloatScaler<Value> {
        FloatScaler { mult }
    }
    fn scale(self: &Self, value: Value) -> Value {
        self.mult * value
    }
}

pub struct IntScaler<Value: PrimInt> {
    mult: Value,
    div: Value,
}
impl<Value: PrimInt> IntScaler<Value> {
    fn new(mult: Value, div: Value) -> IntScaler<Value> {
        IntScaler { mult, div }
    }
    fn scale(self: &Self, value: Value) -> Value {
        (self.mult * value) / self.div
    }
}

pub struct ShiftScaler<Value: PrimInt> {
    mult: Value,
    num_bits: u8,
}
impl<Value: PrimInt> ShiftScaler<Value> {
    fn new(mult: Value, num_bits: u8) -> ShiftScaler<Value> {
        ShiftScaler { mult, num_bits }
    }
    fn scale(self: &Self, value: Value) -> Value {
        (self.mult * value) << self.num_bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn identity1() {
        let scaler: IdentityScaler<i32> = IdentityScaler::new();
        let x = 7;
        let y = scaler.scale(x);
        assert_eq!(y, x);
    }

    #[test]
    fn float_scaler1() {
        let scaler = FloatScaler::new(2.5);
        let x = 7.8;
        let y = scaler.scale(x);
        assert_eq!(y, 2.5 * 7.8);
    }

    #[test]
    fn int_scaler1() {
        let scaler = IntScaler::new(37, 65536);
        let x = 32768;
        let y = scaler.scale(x);
        assert_eq!(y, 18);
    }

}
