pub mod binary_matrix_encryption {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;
    
    pub struct SplitedCharacter {
        pub heavyweight_bits: u8,
        pub lightweight_bits: u8
    }

    impl SplitedCharacter {
        pub fn split_character(&mut self, character: char) {
            self.heavyweight_bits = character as u8 & 0b11110000;
            self.lightweight_bits = character as u8 & 0b00001111;
        }

        pub fn split_number(&mut self, character: u8) {
            self.heavyweight_bits = character & 0b11110000;
            self.lightweight_bits = character & 0b00001111;
        }
    }

    pub fn read_file(filename: String) -> String {
        let file = File::open(filename).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        return contents;
    }

    pub fn string_to_number(string: String) -> Vec<u8> {
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

        for _i in 1..enumerate_index {
            result *= 2;
        }

        return result;
    }

    pub fn binary_string_to_number(binary_string: String) -> Result<u8, &'static str> {
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
            .filter(|(_i, character)| character.to_digit(2).unwrap() == 1)
            .fold(0, |acc, (i, _x)| acc + compute_binary_value(i  as u8));

        return Ok(number_convertion);
    }

    pub fn get_secret_key_index(key_matrix: &[String; 4]) -> Result<[i8; 4], &'static str> {
        if ! key_is_well_form(key_matrix) {
            return Err("The matrix key doesn't conform to the specifications");
        }

        let mut secret_lines: [i8; 4] = [-1; 4];

        for i in 0..7 {
            let current_bits: [char; 4] = [ 
                key_matrix[0].as_bytes()[i] as char,
                key_matrix[1].as_bytes()[i] as char,
                key_matrix[2].as_bytes()[i] as char,
                key_matrix[3].as_bytes()[i] as char
            ];

            let index_detected_secret_line = match check_secret_lines(current_bits) {
                Some(index_value) => index_value as i8,
                None => -1,
            };
            
            if index_detected_secret_line == -1 {
                continue;
            }

            if secret_lines[index_detected_secret_line as usize] != -1 {
                return Err("The key desn't have a clear sequence of identity lines");
            }

            secret_lines[index_detected_secret_line as usize] = i as i8;
        }

        return Ok(secret_lines);
    }

    pub fn key_is_well_form(key_matrix: &[String; 4]) -> bool {
        if key_matrix.len() != 4 {
            return false;
        }

        let good_number_bits = key_matrix.iter()
            .filter(|matrix_line| matrix_line.len() != 8)
            .fold(false, |acc, _x| acc == true);
        
        if ! good_number_bits {
            return false;
        }

        return true;
    }

    pub fn check_secret_lines(current_bits: [char; 4]) -> Option<u8> {
        let first_line_secret_pattern = ['1', '0', '0', '0'];
        let second_line_secret_pattern = ['0', '1', '0', '0'];
        let third_line_secret_pattern = ['0', '0', '1', '0'];
        let fourth_line_secret_pattern = ['0', '0', '0', '1'];

        if current_bits == first_line_secret_pattern {
            return Some(0);

        }else if current_bits == second_line_secret_pattern {
            return Some(1);

        }else if current_bits == third_line_secret_pattern {
            return Some(2);

        }else if current_bits == fourth_line_secret_pattern {
            return Some(3);
        }

        return None;

    }

    // let splited_as_8bits_string: Vec<String> = binary_string
    // .split_whitespace()
    // .map(|element| String::from(element))
    // .collect();

    // pub fn read_key(filename: String) {

    // }

    // fn key_is_valid() -> bool {
    //     return false;
    // }

    // pub fn encrypt_file(filename: String) {

    // }

    // pub fn decrypt_file(filename: String) {

    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn read_file_test() {
        let test_value: String = crate::binary_matrix_encryption::read_file(String::from("test.txt"));
        assert_eq!(test_value, "coucou");
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn read_file_test_empty() {
        crate::binary_matrix_encryption::read_file(String::from(""));
    }
    
    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn read_file_test_not_exist() {
        crate::binary_matrix_encryption::read_file(String::from("azertyuiiop.txt"));
    }

    #[test]
    fn string_to_number_test() {
        let test_value = crate::binary_matrix_encryption::string_to_number(String::from("coucou"));
        assert_eq!(test_value, [99, 111, 117, 99, 111, 117]);
    }

    #[test]
    fn string_to_number_empty() {
        let test_value = crate::binary_matrix_encryption::string_to_number(String::from(""));
        assert_eq!(test_value, []);
    }

    #[test]
    fn compute_binary_value_test() {
        let test_value = crate::binary_matrix_encryption::compute_binary_value(7);
        assert_eq!(test_value, 128);
    }

    #[test]
    fn compute_binary_value_0() {
        let test_value = crate::binary_matrix_encryption::compute_binary_value(0);
        assert_eq!(test_value, 1);
    }

    #[test]
    fn binary_string_to_number_test() {
        let test_value = crate::binary_matrix_encryption::binary_string_to_number(String::from("01101111")).unwrap();
        assert_eq!(test_value, 111);
    }

    #[test]
    #[should_panic(expected = "Can only convert a 8 bits binary string")]
    fn binary_string_to_number_empty() {
        let test_value = crate::binary_matrix_encryption::binary_string_to_number(String::from("")).unwrap();
        assert_eq!(test_value, 111);
    }

    #[test]
    #[should_panic(expected = "Can only convert a 8 bits binary string")]
    fn binary_string_to_number_too_many_character() {
        let test_value = crate::binary_matrix_encryption::binary_string_to_number(String::from("000000000")).unwrap();
        assert_eq!(test_value, 111);
    }

    #[test]
    #[should_panic(expected = "Can only convert a 8 bits binary string")]
    fn binary_string_to_number_not_enough_character() {
        let test_value = crate::binary_matrix_encryption::binary_string_to_number(String::from("0000000")).unwrap();
        assert_eq!(test_value, 111);
    }

    #[test]
    fn binary_string_to_number_zero() {
        let test_value = crate::binary_matrix_encryption::binary_string_to_number(String::from("00000000")).unwrap();
        assert_eq!(test_value, 0);
    }

    #[test]
    fn binary_string_to_number_full() {
        let test_value = crate::binary_matrix_encryption::binary_string_to_number(String::from("11111111")).unwrap();
        assert_eq!(test_value, 255);
    }

    #[test]
    fn splited_character_split_character_test() {
        let mut character = crate::binary_matrix_encryption::SplitedCharacter{heavyweight_bits: 0, lightweight_bits: 0};
        character.split_character('a');
        assert_eq!(character.heavyweight_bits, 96);
        assert_eq!(character.lightweight_bits, 1);
    }

    #[test]
    fn splited_character_split_number_test() {
        let mut character = crate::binary_matrix_encryption::SplitedCharacter{heavyweight_bits: 0, lightweight_bits: 0};
        character.split_number(97);
        assert_eq!(character.heavyweight_bits, 96);
        assert_eq!(character.lightweight_bits, 1);
    }
}
