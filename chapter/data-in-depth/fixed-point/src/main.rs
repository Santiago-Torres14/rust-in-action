//! Rust in action
//! chapter 5
//! fixed-point number format

fn main() {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
    fn from(n: f64) -> Self {
        if n >= 1.0 {
            Q7(127)
        } else if n <= -1.0 {
            Q7(-128)
        } else {
            Q7((n * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(value: Q7) -> Self {
        // TODO!: investigate why
        (value.0 as f64) * 2_f64.powf(-7.0)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(<f64 as Into<Q7>>::into(10.), Q7::from(1.));
    }
}
