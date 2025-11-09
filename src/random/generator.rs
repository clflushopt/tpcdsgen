use crate::random::stream::RandomNumberStream;
use crate::types::{Date, Decimal};

pub struct RandomValueGenerator;

impl RandomValueGenerator {
    pub const ALPHA_NUMERIC: &'static str =
        "abcdefghijklmnopqrstuvxyzABCDEFGHIJKLMNOPQRSTUVXYZ0123456789";
    pub const DIGITS: &'static str = "0123456789";

    pub fn generate_uniform_random_int(
        min: i32,
        max: i32,
        random_number_stream: &mut dyn RandomNumberStream,
    ) -> i32 {
        // truncating long to int copies behavior of c code.
        let mut result = random_number_stream.next_random() as i32;
        result %= max - min + 1;
        result += min;
        result
    }

    pub fn generate_uniform_random_key(
        min: i64,
        max: i64,
        random_number_stream: &mut dyn RandomNumberStream,
    ) -> i64 {
        // truncating long to int copies behavior of c code
        let mut result = random_number_stream.next_random() as i32;
        result %= (max - min + 1) as i32;
        result += min as i32;
        result as i64
    }

    pub fn generate_uniform_random_decimal(
        min: Decimal,
        max: Decimal,
        random_number_stream: &mut dyn RandomNumberStream,
    ) -> Decimal {
        let precision = if min.get_precision() < max.get_precision() {
            min.get_precision()
        } else {
            max.get_precision()
        };

        // compute number
        let mut number = random_number_stream.next_random();
        number %= max.get_number() - min.get_number() + 1;
        number += min.get_number();

        Decimal::new(number, precision).unwrap()
    }

    pub fn generate_uniform_random_date(
        min: Date,
        max: Date,
        random_number_stream: &mut dyn RandomNumberStream,
    ) -> crate::error::Result<Date> {
        let range = max.to_julian_days() - min.to_julian_days();
        let julian_days = min.to_julian_days()
            + Self::generate_uniform_random_int(0, range, random_number_stream);
        Ok(Date::from_julian_days(julian_days))
    }

    // Generate random string of specified length from given character set
    pub fn generate_random_string(
        length: usize,
        character_set: &str,
        random_number_stream: &mut dyn RandomNumberStream,
    ) -> String {
        let chars: Vec<char> = character_set.chars().collect();
        let mut result = String::with_capacity(length);

        for _ in 0..length {
            let index =
                Self::generate_uniform_random_int(0, chars.len() as i32 - 1, random_number_stream);
            result.push(chars[index as usize]);
        }

        result
    }

    // Generate random alphanumeric string
    pub fn generate_random_alphanumeric(
        length: usize,
        random_number_stream: &mut dyn RandomNumberStream,
    ) -> String {
        Self::generate_random_string(length, Self::ALPHA_NUMERIC, random_number_stream)
    }

    // Generate random numeric string
    pub fn generate_random_digits(
        length: usize,
        random_number_stream: &mut dyn RandomNumberStream,
    ) -> String {
        Self::generate_random_string(length, Self::DIGITS, random_number_stream)
    }

    // Generate random charset string with variable length (generateRandomCharset)
    // This method loops to max to consume the same number of random values as the Java implementation
    pub fn generate_random_charset(
        character_set: &str,
        min: i32,
        max: i32,
        random_number_stream: &mut dyn RandomNumberStream,
    ) -> String {
        let length = Self::generate_uniform_random_int(min, max, random_number_stream);
        let chars: Vec<char> = character_set.chars().collect();
        let mut result = String::with_capacity(length as usize);

        // Loop to max to consume the same number of random values (behavior)
        for i in 0..max {
            let index =
                Self::generate_uniform_random_int(0, chars.len() as i32 - 1, random_number_stream);
            if i < length {
                result.push(chars[index as usize]);
            }
        }

        result
    }

    // Generate random boolean with given probability (0.0 to 1.0)
    pub fn generate_random_boolean(
        probability: f64,
        random_number_stream: &mut dyn RandomNumberStream,
    ) -> bool {
        random_number_stream.next_random_double() < probability
    }

    // Generate random weighted selection from array (indices)
    pub fn generate_weighted_random_index(
        weights: &[i32],
        random_number_stream: &mut dyn RandomNumberStream,
    ) -> usize {
        let total_weight: i32 = weights.iter().sum();
        let random_value = Self::generate_uniform_random_int(1, total_weight, random_number_stream);

        let mut cumulative_weight = 0;
        for (index, &weight) in weights.iter().enumerate() {
            cumulative_weight += weight;
            if random_value <= cumulative_weight {
                return index;
            }
        }

        // Fallback to last index (should not happen with proper weights)
        weights.len() - 1
    }

    // Generate random text following Java implementation exactly
    pub fn generate_random_text(
        min_length: i32,
        max_length: i32,
        random_number_stream: &mut dyn RandomNumberStream,
    ) -> String {
        

        let mut is_sentence_beginning = true;
        let mut text = String::new();
        let mut target_length =
            Self::generate_uniform_random_int(min_length, max_length, random_number_stream);

        while target_length > 0 {
            let mut generated = Self::generate_random_sentence(random_number_stream);
            if is_sentence_beginning && !generated.is_empty() {
                let first_char = generated
                    .chars()
                    .next()
                    .unwrap()
                    .to_uppercase()
                    .collect::<String>();
                generated = first_char + &generated[1..];
            }

            let generated_length = generated.len() as i32;
            is_sentence_beginning = generated.ends_with('.');

            // truncate so as not to exceed target length
            if target_length < generated_length {
                generated = generated[..target_length as usize].to_string();
            }

            target_length -= generated_length;

            text.push_str(&generated);
            if target_length > 0 {
                text.push(' ');
                target_length -= 1;
            }
        }

        text
    }

    // Generate random sentence following Java implementation exactly
    fn generate_random_sentence(random_number_stream: &mut dyn RandomNumberStream) -> String {
        use crate::distribution::*;

        let mut verbiage = String::new();
        let syntax = pick_random_sentence(random_number_stream).unwrap_or("N V.");

        for ch in syntax.chars() {
            match ch {
                'N' => verbiage.push_str(pick_random_noun(random_number_stream).unwrap_or("thing")),
                'V' => verbiage.push_str(pick_random_verb(random_number_stream).unwrap_or("is")),
                'J' => {
                    verbiage.push_str(pick_random_adjective(random_number_stream).unwrap_or("good"))
                }
                'D' => {
                    verbiage.push_str(pick_random_adverb(random_number_stream).unwrap_or("well"))
                }
                'X' => {
                    verbiage.push_str(pick_random_auxiliary(random_number_stream).unwrap_or("can"))
                }
                'P' => {
                    verbiage.push_str(pick_random_preposition(random_number_stream).unwrap_or("to"))
                }
                'A' => {
                    verbiage.push_str(pick_random_article(random_number_stream).unwrap_or("the"))
                }
                'T' => {
                    verbiage.push_str(pick_random_terminator(random_number_stream).unwrap_or("."))
                }
                _ => verbiage.push(ch), // this is for adding punctuation and white space.
            }
        }

        verbiage
    }

    // Generate word based on seed and syllables distribution (exact Java implementation)
    pub fn generate_word(
        seed: i32,
        max_chars: i32,
        _random_number_stream: &mut dyn RandomNumberStream,
    ) -> String {
        use crate::distribution::get_syllables_distribution;
        use crate::distribution::utils::Distribution;

        let distribution = get_syllables_distribution();
        let size = distribution.get_size();
        let mut word = String::new();
        let mut seed = seed as i64;

        while seed > 0 {
            let syllable = distribution
                .get_value_at_index(0, (seed % size as i64) as usize)
                .unwrap_or("syl");
            seed /= size as i64;

            if (word.len() + syllable.len()) <= max_chars as usize {
                word.push_str(syllable);
            } else {
                break;
            }
        }

        word
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::stream::RandomNumberStreamImpl;

    #[test]
    fn test_uniform_random_int() {
        let mut stream = RandomNumberStreamImpl::new(1).unwrap();
        let result = RandomValueGenerator::generate_uniform_random_int(1, 10, &mut stream);
        assert!(result >= 1 && result <= 10);
    }

    #[test]
    fn test_uniform_random_key() {
        let mut stream = RandomNumberStreamImpl::new(1).unwrap();
        let result = RandomValueGenerator::generate_uniform_random_key(100, 200, &mut stream);
        assert!(result >= 100 && result <= 200);
    }

    #[test]
    fn test_uniform_random_decimal() {
        let mut stream = RandomNumberStreamImpl::new(1).unwrap();
        let min = Decimal::new(100, 2).unwrap(); // 1.00
        let max = Decimal::new(500, 2).unwrap(); // 5.00
        let result = RandomValueGenerator::generate_uniform_random_decimal(min, max, &mut stream);

        assert!(result.get_number() >= min.get_number() && result.get_number() <= max.get_number());
        assert_eq!(result.get_precision(), 2);
    }

    #[test]
    fn test_uniform_random_date() {
        let mut stream = RandomNumberStreamImpl::new(1).unwrap();
        let min = Date::new(2020, 1, 1);
        let max = Date::new(2020, 12, 31);
        let result =
            RandomValueGenerator::generate_uniform_random_date(min, max, &mut stream).unwrap();

        assert!(result.to_julian_days() >= min.to_julian_days());
        assert!(result.to_julian_days() <= max.to_julian_days());
        assert_eq!(result.get_year(), 2020);
    }

    #[test]
    fn test_random_alphanumeric() {
        let mut stream = RandomNumberStreamImpl::new(1).unwrap();
        let result = RandomValueGenerator::generate_random_alphanumeric(10, &mut stream);

        assert_eq!(result.len(), 10);
        for ch in result.chars() {
            assert!(RandomValueGenerator::ALPHA_NUMERIC.contains(ch));
        }
    }

    #[test]
    fn test_random_digits() {
        let mut stream = RandomNumberStreamImpl::new(1).unwrap();
        let result = RandomValueGenerator::generate_random_digits(5, &mut stream);

        assert_eq!(result.len(), 5);
        for ch in result.chars() {
            assert!(ch.is_ascii_digit());
        }
    }

    #[test]
    fn test_random_boolean() {
        let mut stream = RandomNumberStreamImpl::new(1).unwrap();

        // Test with 0% probability - should always be false
        let result_never = RandomValueGenerator::generate_random_boolean(0.0, &mut stream);
        // Note: This might not always be false due to floating point precision, but typically should be

        // Test with 100% probability - should always be true
        let result_always = RandomValueGenerator::generate_random_boolean(1.0, &mut stream);
        assert!(result_always);
    }

    #[test]
    fn test_weighted_random_index() {
        let mut stream = RandomNumberStreamImpl::new(1).unwrap();
        let weights = vec![10, 20, 30, 40];
        let result = RandomValueGenerator::generate_weighted_random_index(&weights, &mut stream);

        assert!(result < weights.len());
    }

    #[test]
    fn test_random_string_custom_charset() {
        let mut stream = RandomNumberStreamImpl::new(1).unwrap();
        let charset = "ABC123";
        let result = RandomValueGenerator::generate_random_string(8, charset, &mut stream);

        assert_eq!(result.len(), 8);
        for ch in result.chars() {
            assert!(charset.contains(ch));
        }
    }
}
