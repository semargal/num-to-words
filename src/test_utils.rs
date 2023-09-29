use crate::errors::Error;
use num::Integer;

pub struct InOut<T: Integer + Copy>(pub T, pub &'static str);

pub fn test_set<T: Integer + Copy>(f: &dyn Fn(T) -> Result<String, Error>, data: &[InOut<T>]) {
    for sample in data.iter() {
        assert_eq!(f(sample.0).unwrap(), sample.1);
    }
}
