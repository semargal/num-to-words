use crate::errors::{Error, ErrorRepr};
use crate::utils::*;
use num::{Integer, NumCast};
use std::ops::Neg;

const UNITS: [&str; 10] = [
    "",
    "один",
    "два",
    "три",
    "чотири",
    "п'ять",
    "шість",
    "сім",
    "вісім",
    "дев'ять",
];

const TEENS: [&str; 10] = [
    "десять",
    "одинадцять",
    "дванадцять",
    "тринадцять",
    "чотирнадцять",
    "п'ятнадцять",
    "шістнадцять",
    "сімнадцять",
    "вісімнадцять",
    "дев'ятнадцять",
];

const TENS: [&str; 10] = [
    "",
    "десять",
    "двадцять",
    "тридцять",
    "сорок",
    "п'ятдесят",
    "шістдесят",
    "сімдесят",
    "вісімдесят",
    "дев'яносто",
];

const HUNDREDS: [&str; 10] = [
    "",
    "сто",
    "двісті",
    "триста",
    "чотириста",
    "п'ятсот",
    "шістсот",
    "сімсот",
    "вісімсот",
    "дев'ятсот",
];

const MEGAS: [[&str; 3]; 10] = [
    ["", "", ""],
    ["тисяча", "тисячі", "тисяч"],                    // 10^3
    ["мільйон", "мільйона", "мільйонів"],             // 10^6
    ["мільярд", "мільярда", "мільярдів"],             // 10^9
    ["трильйон", "трильйона", "трильйонів"],          // 10^12
    ["квадрильйон", "квадрильйона", "квадрильйонів"], // 10^15
    ["квінтильйон", "квінтильйона", "квінтильйонів"], // 10^18
    ["секстильйон", "секстильйона", "секстильйонів"], // 10^21
    ["септильйон", "септильйона", "септильйонів"],    // 10^34
    ["октильйон", "октильйона", "октильйонів"],       // 10^27
];

/// Converts integers to Ukrainian
pub fn integer_to_uk_ua<T: Integer + NumCast + Neg<Output = T> + Copy>(
    mut input: T,
) -> Result<String, Error> {
    let mut words: Vec<String> = vec![];

    if input < T::zero() {
        words.push("мінус".into());
        input = input.neg();
    }

    // Split integer in triplets
    let triplets = int_to_triplets(input)?;

    // Zero is a special case
    if triplets.is_empty() {
        return Ok("нуль".into());
    }

    let zero = T::zero().to_usize().ok_or(ErrorRepr::IntToUsizeError)?;
    let one = T::one().to_usize().ok_or(ErrorRepr::IntToUsizeError)?;
    let ten = T::from(10).ok_or(ErrorRepr::IntToGenError)?;
    let ten_usize = ten.to_usize().ok_or(ErrorRepr::IntToUsizeError)?;
    let hundred = T::from(100).ok_or(ErrorRepr::IntToGenError)?;

    // Iterate over triplets
    for (idx, triplet) in triplets.iter().enumerate().rev() {
        if triplet == &T::zero() {
            continue;
        }

        // Three digits
        let hundreds = (*triplet / hundred % ten)
            .to_usize()
            .ok_or(ErrorRepr::IntToUsizeError)?;
        let mut tens = (*triplet / ten % ten)
            .to_usize()
            .ok_or(ErrorRepr::IntToUsizeError)?;
        let units = (*triplet % ten)
            .to_usize()
            .ok_or(ErrorRepr::IntToUsizeError)?;

        if hundreds > zero {
            words.push(HUNDREDS[hundreds].into());
        }

        if tens != zero || units != zero {
            match tens {
                0 => {
                    words.push(fix_one_two_unit(units, idx, &UNITS));
                }
                1 => {
                    words.push(TEENS[units].into());
                }
                _ => {
                    words.push(TENS[tens].into());

                    if units > 0 {
                        words.push(fix_one_two_unit(units, idx, &UNITS))
                    }
                }
            }
        }

        // Mega
        if idx >= one && idx < MEGAS.len() {
            let mega = MEGAS[idx];
            tens = tens * ten_usize + units;

            if !mega.is_empty() {
                words.push(plural(tens, &mega));
            }
        }
    }

    Ok(words.join(" "))
}

fn plural(mut n: usize, words: &[&str]) -> String {
    let mut index = 0;

    n %= 100;

    if n > 19 {
        n %= 10
    }

    if n != 1 {
        if n > 1 && n <= 4 {
            index = 1
        } else {
            index = 2
        }
    }

    words[index].into()
}

fn fix_one_two_unit(unit: usize, idx: usize, arr: &[&str]) -> String {
    if idx == 1 {
        match unit {
            1 => return "одна".into(),
            2 => return "дві".into(),
            _ => (),
        }
    }

    arr[unit].into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    const TEST_SET: [InOut<i64>; 64] = [
		InOut(-1, "мінус один"),
		InOut(0, "нуль"),
		InOut(1, "один"),
		InOut(9, "дев'ять"),
		InOut(10, "десять"),
		InOut(11, "одинадцять"),
		InOut(19, "дев'ятнадцять"),
		InOut(20, "двадцять"),
		InOut(21, "двадцять один"),
		InOut(80, "вісімдесят"),
		InOut(90, "дев'яносто"),
		InOut(99, "дев'яносто дев'ять"),
		InOut(100, "сто"),
		InOut(101, "сто один"),
		InOut(111, "сто одинадцять"),
		InOut(120, "сто двадцять"),
		InOut(121, "сто двадцять один"),
		InOut(900, "дев'ятсот"),
		InOut(909, "дев'ятсот дев'ять"),
		InOut(919, "дев'ятсот дев'ятнадцять"),
		InOut(990, "дев'ятсот дев'яносто"),
		InOut(999, "дев'ятсот дев'яносто дев'ять"),
		InOut(1000, "одна тисяча"),
		InOut(2000, "дві тисячі"),
		InOut(4000, "чотири тисячі"),
		InOut(5000, "п'ять тисяч"),
		InOut(11000, "одинадцять тисяч"),
		InOut(21000, "двадцять одна тисяча"),
		InOut(999000, "дев'ятсот дев'яносто дев'ять тисяч"),
		InOut(999999, "дев'ятсот дев'яносто дев'ять тисяч дев'ятсот дев'яносто дев'ять"),
		InOut(1000000, "один мільйон"),
		InOut(2000000, "два мільйона"),
		InOut(4000000, "чотири мільйона"),
		InOut(5000000, "п'ять мільйонів"),
		InOut(100100100, "сто мільйонів сто тисяч сто"),
		InOut(500500500, "п'ятсот мільйонів п'ятсот тисяч п'ятсот"),
		InOut(606606606, "шістсот шість мільйонів шістсот шість тисяч шістсот шість"),
		InOut(999000000, "дев'ятсот дев'яносто дев'ять мільйонів"),
		InOut(999000999, "дев'ятсот дев'яносто дев'ять мільйонів дев'ятсот дев'яносто дев'ять"),
		InOut(999999000, "дев'ятсот дев'яносто дев'ять мільйонів дев'ятсот дев'яносто дев'ять тисяч"),
		InOut(999999999, "дев'ятсот дев'яносто дев'ять мільйонів дев'ятсот дев'яносто дев'ять тисяч дев'ятсот дев'яносто дев'ять"),
		InOut(1174315110, "один мільярд сто сімдесят чотири мільйона триста п'ятнадцять тисяч сто десять"),
		InOut(1174315119, "один мільярд сто сімдесят чотири мільйона триста п'ятнадцять тисяч сто дев'ятнадцять"),
		InOut(15174315119, "п'ятнадцять мільярдів сто сімдесят чотири мільйона триста п'ятнадцять тисяч сто дев'ятнадцять"),
		InOut(35174315119, "тридцять п'ять мільярдів сто сімдесят чотири мільйона триста п'ятнадцять тисяч сто дев'ятнадцять"),
		InOut(935174315119, "дев'ятсот тридцять п'ять мільярдів сто сімдесят чотири мільйона триста п'ятнадцять тисяч сто дев'ятнадцять"),
		InOut(2935174315119, "два трильйона дев'ятсот тридцять п'ять мільярдів сто сімдесят чотири мільйона триста п'ятнадцять тисяч сто дев'ятнадцять"),
		InOut(3911760, "три мільйона дев'ятсот одинадцять тисяч сімсот шістдесят"),
		InOut(27, "двадцять сім"),
		InOut(95000001000, "дев'яносто п'ять мільярдів одна тисяча"),
		InOut(57482, "п'ятдесят сім тисяч чотириста вісімдесят два"),
		InOut(5, "п'ять"),
		InOut(16, "шістнадцять"),
		InOut(30, "тридцять"),
		InOut(53, "п'ятдесят три"),
		InOut(123, "сто двадцять три"),
		InOut(204, "двісті чотири"),
		InOut(300, "триста"),
		InOut(1400, "одна тисяча чотириста"),
		InOut(83756, "вісімдесят три тисячі сімсот п'ятдесят шість"),
		InOut(293111, "двісті дев'яносто три тисячі сто одинадцять"),
		InOut(32001950, "тридцять два мільйона одна тисяча дев'ятсот п'ятдесят"),
		InOut(2018, "дві тисячі вісімнадцять"),
		InOut(126682, "сто двадцять шість тисяч шістсот вісімдесят два"),
    ];

    #[cfg(feature = "uk_ua")]
    #[test]
    fn uk_ua() {
        test_set(&integer_to_uk_ua, &TEST_SET);
    }
}
