use crate::errors::{Error, ErrorRepr};
use num::{Integer, NumCast};

pub fn int_to_triplets<T: Integer + NumCast + Copy>(mut number: T) -> Result<Vec<T>, Error> {
    let mut triplets = Vec::new();
    let thousand = T::from(1000).ok_or(ErrorRepr::IntToGenError)?;

    while number > T::zero() {
        triplets.push(number % thousand);
        number = number / thousand;
    }
    Ok(triplets)
}
