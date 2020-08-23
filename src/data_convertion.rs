#[derive(Debug, Clone, Copy)]
pub struct SplitedCharacter {
    pub heavyweight_bits: Option<u8>,
    pub lightweight_bits: Option<u8>
}

impl SplitedCharacter {
    pub fn new() -> Self {
        return Self {
            heavyweight_bits: None,
            lightweight_bits: None
        }; 
    }

    pub fn from_character(character: &char) -> Self {
        return Self {
            heavyweight_bits: Some(*character as u8 & 0b11110000),
            lightweight_bits: Some(*character as u8 & 0b00001111)
        };
    }

    pub fn from_number(character: &u8) -> Self {
        return Self {
            heavyweight_bits: Some(*character & 0b11110000),
            lightweight_bits: Some(*character & 0b00001111)
        } 
    }

    pub fn get_value(self) -> Option<char> {
        if self.heavyweight_bits.is_none() || self.lightweight_bits.is_none() {
            return None;
        }

        return Some((self.heavyweight_bits.unwrap() + self.lightweight_bits.unwrap()) as char)
    }
}


pub fn string_to_number(string: &str) -> Vec<u8> {
    let binary_data = string.chars()
        .map(|character| character as u8)
        .collect();
    
    return binary_data;
}

pub fn compute_binary_value(enumerate_index: u8) -> u8 {
    if enumerate_index == 0 {
        return 1;
    }

    let mut result = 2;

    for _ in 1..enumerate_index {
        result *= 2;
    }

    return result;
}

pub fn binary_string_to_number(binary_string: &str) -> Result<u8, &'static str> {
    if binary_string.len() != 8 {
        return Err("Can only convert a 8 bits binary string");
    }

    let is_well_formed = binary_string.chars()
        .all(|character| character.is_digit(2));

    if ! is_well_formed {
        return Err("Is not a binary string")
    }

    let number_convertion: u8 = binary_string.chars()
        .rev()
        .enumerate()
        .filter(|(_, character)| character.to_digit(2).unwrap() == 1)
        .fold(0, |acc, (i, _)| acc + compute_binary_value(i  as u8));

    return Ok(number_convertion);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_to_number_test() {
        let test_value = string_to_number("coucou");
        assert_eq!(test_value, [99, 111, 117, 99, 111, 117]);
    }
    
    #[test]
    fn string_to_number_empty() {
        let test_value = string_to_number("");
        assert_eq!(test_value, []);
    }
    
    #[test]
    fn compute_binary_value_test() {
        let test_value = compute_binary_value(7);
        assert_eq!(test_value, 128);
    }
    
    #[test]
    fn compute_binary_value_0() {
        let test_value = compute_binary_value(0);
        assert_eq!(test_value, 1);
    }
    
    #[test]
    fn binary_string_to_number_test() {
        let test_value = binary_string_to_number("01101111").unwrap();
        assert_eq!(test_value, 111);
    }
    
    #[test]
    #[should_panic(expected = "Can only convert a 8 bits binary string")]
    fn binary_string_to_number_empty() {
        let _test_value = binary_string_to_number("").unwrap();
    }
    
    #[test]
    #[should_panic(expected = "Can only convert a 8 bits binary string")]
    fn binary_string_to_number_too_many_character() {
        let _test_value = binary_string_to_number("000000000").unwrap();
    }
    
    #[test]
    #[should_panic(expected = "Can only convert a 8 bits binary string")]
    fn binary_string_to_number_not_enough_character() {
        let _test_value = binary_string_to_number("0000000").unwrap();
    }
    
    #[test]
    fn binary_string_to_number_zero() {
        let test_value = binary_string_to_number("00000000").unwrap();
        assert_eq!(test_value, 0);
    }
    
    #[test]
    fn binary_string_to_number_full() {
        let test_value = binary_string_to_number("11111111").unwrap();
        assert_eq!(test_value, 255);
    }
    
    #[test]
    fn splited_character_from_character_test() {
        let character = SplitedCharacter::from_character(&'a');

        assert_eq!(character.heavyweight_bits.unwrap(), 96);
        assert_eq!(character.lightweight_bits.unwrap(), 1);
    }
    
    #[test]
    fn splited_character_from_number_test() {
        let character = SplitedCharacter::from_number(&9);

        assert_eq!(character.heavyweight_bits.unwrap(), 0);
        assert_eq!(character.lightweight_bits.unwrap(), 9);
    }

    #[test]
    fn splited_character_get_value() {
        let character = SplitedCharacter::from_character(&'b');

        assert_eq!(character.get_value().unwrap(), 'b');
    }
}
