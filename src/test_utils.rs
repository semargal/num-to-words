use crate::types::*;

pub struct InOut(pub Int, pub StaticStr);

pub fn test_set(f: &dyn Fn(Int) -> Result<String>, data: &[InOut]) {
    for sample in data.iter() {
        assert_eq!(f(sample.0).unwrap(), sample.1);
    }
}
