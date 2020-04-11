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

    pub fn read_file(filename: &str) -> String {
        let file = File::open(filename).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        return contents;
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

        for _i in 1..enumerate_index {
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
            .filter(|(_i, character)| character.to_digit(2).unwrap() == 1)
            .fold(0, |acc, (i, _x)| acc + compute_binary_value(i  as u8));

        return Ok(number_convertion);
    }

    pub fn get_secret_key_index(key_matrix: &[&str; 4]) -> Result<[i8; 4], &'static str> {
        if ! key_is_well_form(key_matrix) {
            return Err("The matrix key doesn't conform to the specifications");
        }

        let mut secret_lines: [i8; 4] = [-1; 4];

        for i in 0..8 {
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

            secret_lines[index_detected_secret_line as usize] = i as i8;
        }

        let all_element_are_set = secret_lines.iter()
            .filter(|index_value| **index_value == -1)
            .fold(true, |acc, _index_value| acc == false);

        if ! all_element_are_set {
            return Err("The key desn't have a clear sequence of identity lines");
        }

        return Ok(secret_lines);
    }

    pub fn key_is_well_form(key_matrix: &[&str; 4]) -> bool {
        let good_number_bits = key_matrix.iter()
            .filter(|matrix_line| matrix_line.len() != 8)
            .fold(true, |acc, _x| acc == false);
        
        if ! good_number_bits {
            return false;
        }

        for matrix_line in key_matrix {
            for character in matrix_line.chars() {
                if ! character.is_digit(2) {
                    return false;
                }
            }
            
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
    use super::binary_matrix_encryption::*;

    #[test]
    fn read_file_test() {
        let test_value: String = read_file("test.txt");
        assert_eq!(test_value, "coucou");
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn read_file_test_empty() {
        read_file("");
    }
    
    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn read_file_test_not_exist() {
        read_file("azertyuiiop.txt");
    }

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
    fn splited_character_split_character_test() {
        let mut character = SplitedCharacter{heavyweight_bits: 0, lightweight_bits: 0};
        character.split_character('a');
        assert_eq!(character.heavyweight_bits, 96);
        assert_eq!(character.lightweight_bits, 1);
    }

    #[test]
    fn splited_character_split_number_test() {
        let mut character = SplitedCharacter{heavyweight_bits: 0, lightweight_bits: 0};
        character.split_number(97);
        assert_eq!(character.heavyweight_bits, 96);
        assert_eq!(character.lightweight_bits, 1);
    }

    #[test]
    fn get_secret_key_index_test() {
        let matrix_test = ["11011000", 
            "11001011", 
            "10001110", 
            "10100000"
        ];

        let test_value = get_secret_key_index(&matrix_test);
        assert_eq!([3, 7, 5, 2], test_value.unwrap());

    }

    #[test]
    #[should_panic]
    fn get_secret_key_index_invalid_key_syntax() {
        let matrix_test = ["11011000", 
            "11001011", 
            "chocolatine", 
            "10100000"
        ];

        get_secret_key_index(&matrix_test).unwrap();
    }

    #[test]
    #[should_panic]
    fn get_secret_key_index_no_secret_found() {
        let matrix_test = ["11011000", 
            "11001011", 
            "10001111", 
            "10100000"
        ];
        
        get_secret_key_index(&matrix_test).unwrap();
    }

    #[test]
    fn key_is_well_form_test() {
        let matrix_test = ["11011000", 
            "11001011", 
            "10001110", 
            "10100000"
        ];
        assert_eq!(key_is_well_form(&matrix_test), true);
    }

    #[test]
    fn key_is_well_form_not_good_number_of_bits() {
        let matrix_test = ["11011000", 
            "11001011", 
            "1000111",
            "10100000"
        ];
        assert_eq!(key_is_well_form(&matrix_test), false);
    }

    #[test]
    fn key_is_well_form_not_binary_string() {
        let matrix_test = ["11011000", 
            "michelfo", 
            "1000111",
            "1010000"
        ];
        assert_eq!(key_is_well_form(&matrix_test), false);
    }

    #[test]
    fn check_secret_lines_test_line_1() {
        let test_value = check_secret_lines(['1', '0', '0', '0']);
        assert_eq!(test_value, Some(0));
    }

    #[test]
    fn check_secret_lines_test_line_2() {
        let test_value = check_secret_lines(['0', '1', '0', '0']);
        assert_eq!(test_value, Some(1));
    }

    #[test]
    fn check_secret_lines_test_line_3() {
        let test_value = check_secret_lines(['0', '0', '1', '0']);
        assert_eq!(test_value, Some(2));
    }

    #[test]
    fn check_secret_lines_test_line_4() {
        let test_value = check_secret_lines(['0', '0', '0', '1']);
        assert_eq!(test_value, Some(3));
    }

    #[test]
    fn check_secret_lines_not_key_line() {
        let test_value = check_secret_lines(['1', '1', '0', '0']);
        assert_eq!(test_value, None);
    }


}
