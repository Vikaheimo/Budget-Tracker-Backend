use nanoid::nanoid;
const CUSTOM_ALPHABET: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
const NUMBERS: usize = 9;

pub fn generate_random_id() -> i32{
    nanoid!(NUMBERS, &CUSTOM_ALPHABET).parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::helpers::user_id_generator::NUMBERS;

    #[test]
    fn test_max_value_is_smaller_than_i32_max() {
        let max_value_str = "9".repeat(NUMBERS);
        let max_value: u64 = max_value_str.parse().unwrap();
        assert!(max_value < i32::MAX as u64)
    }
}