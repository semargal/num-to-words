use crate::types::*;
use crate::utils::*;

const UNITS: [StaticStr; 10] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const TENS: [StaticStr; 10] = [
    "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const TEENS: [StaticStr; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const MEGAS: [StaticStr; 16] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
    "sextillion",
    "septillion",
    "octillion",
    "nonillion",
    "decillion",
    "undecillion",
    "duodecillion",
    "tredecillion",
    "quattuordecillion",
];

/// Converts integers to English
pub fn integer_to_en_us(mut input: Int) -> Result<String> {
    let mut words: Vec<String> = vec![];

    if input < 0 {
        words.push("minus".into());
        input *= -1;
    }

    let triplets = int_to_triplets(input);

    // Zero is a special case
    if triplets.is_empty() {
        return Ok("zero".into());
    }
    // Iterate over triplets
    for (idx, triplet) in triplets.iter().enumerate().rev() {
        if triplet == &0 {
            continue;
        }

        // Three digits
        let hundreds = (triplet / 100 % 10) as usize;
        let tens = (triplet / 10 % 10) as usize;
        let units = (triplet % 10) as usize;

        if hundreds > 0 {
            words.push(UNITS[hundreds].into());
            words.push("hundred".into());
        }

        if tens != 0 || units != 0 {
            match tens {
                0 => {
                    words.push(UNITS[units].into());
                }
                1 => {
                    words.push(TEENS[units].into());
                }
                _ => {
                    let mut ten: String = TENS[tens].into();
                    if units > 0 {
                        ten = format!("{}-{}", ten, UNITS[units]);
                    }
                    words.push(ten);
                }
            }
        }

        // Mega
        let mega = MEGAS[idx];
        if !mega.is_empty() {
            words.push(mega.into());
        }
    }

    Ok(words.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    const TEST_SET: [InOut; 47] =
        [
            InOut(-1, "minus one"),
            InOut(0, "zero"),
            InOut(1, "one"),
            InOut(9, "nine"),
            InOut(10, "ten"),
            InOut(11, "eleven"),
            InOut(19, "nineteen"),
            InOut(20, "twenty"),
            InOut(21, "twenty-one"),
            InOut(80, "eighty"),
            InOut(90, "ninety"),
            InOut(99, "ninety-nine"),
            InOut(100, "one hundred"),
            InOut(101, "one hundred one"),
            InOut(111, "one hundred eleven"),
            InOut(120, "one hundred twenty"),
            InOut(121, "one hundred twenty-one"),
            InOut(900, "nine hundred"),
            InOut(909, "nine hundred nine"),
            InOut(919, "nine hundred nineteen"),
            InOut(990, "nine hundred ninety"),
            InOut(999, "nine hundred ninety-nine"),
            InOut(1000, "one thousand"),
            InOut(2000, "two thousand"),
            InOut(4000, "four thousand"),
            InOut(5000, "five thousand"),
            InOut(11000, "eleven thousand"),
            InOut(21000, "twenty-one thousand"),
            InOut(999000, "nine hundred ninety-nine thousand"),
            InOut(999999, "nine hundred ninety-nine thousand nine hundred ninety-nine"),
            InOut(1000000, "one million"),
            InOut(2000000, "two million"),
            InOut(4000000, "four million"),
            InOut(5000000, "five million"),
            InOut(100100100, "one hundred million one hundred thousand one hundred"),
            InOut(500500500, "five hundred million five hundred thousand five hundred"),
            InOut(606606606, "six hundred six million six hundred six thousand six hundred six"),
            InOut(999000000, "nine hundred ninety-nine million"),
            InOut(999000999, "nine hundred ninety-nine million nine hundred ninety-nine"),
            InOut(999999000, "nine hundred ninety-nine million nine hundred ninety-nine thousand"),
            InOut(999999999, "nine hundred ninety-nine million nine hundred ninety-nine thousand nine hundred ninety-nine"),
            InOut(1174315110, "one billion one hundred seventy-four million three hundred fifteen thousand one hundred ten"),
            InOut(1174315119, "one billion one hundred seventy-four million three hundred fifteen thousand one hundred nineteen"),
            InOut(15174315119, "fifteen billion one hundred seventy-four million three hundred fifteen thousand one hundred nineteen"),
            InOut(35174315119, "thirty-five billion one hundred seventy-four million three hundred fifteen thousand one hundred nineteen"),
            InOut(935174315119, "nine hundred thirty-five billion one hundred seventy-four million three hundred fifteen thousand one hundred nineteen"),
            InOut(2935174315119, "two trillion nine hundred thirty-five billion one hundred seventy-four million three hundred fifteen thousand one hundred nineteen"),
        ];

    #[cfg(feature = "en_us")]
    #[test]
    fn en_us() {
        test_set(&integer_to_en_us, &TEST_SET);
    }
}
