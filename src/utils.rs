use crate::types::*;

pub fn int_to_triplets(mut number: Int) -> Vec<Int> {
    let mut triplets = Vec::new();

    while number > 0 {
        triplets.push(number % 1000);
        number /= 1000;
    }
    triplets
}
