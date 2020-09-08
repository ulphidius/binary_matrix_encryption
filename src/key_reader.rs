use std::fs::File;
use std::io::BufReader;
use std::io::prelude::Read;

fn key_file_is_well_form(key_file_content: &String) -> bool {
    let file_size: usize = 41;

    if key_file_content.len() != file_size {
        return false;
    }

    let mut key_file_specific_formation: Vec<char> = key_file_content.chars()
        .enumerate()
        .filter(|(index, _)| index < &5)
        .map(|(_, character)| character.clone())
        .collect();
    
    key_file_specific_formation.push(key_file_content.chars().last().unwrap());

    if key_file_specific_formation.into_iter().collect::<String>() != "G4C=[]" {
        return false;    
    }

    return true;
}

pub fn read_key(filename: &str) -> Result<String, &'static str> {
    let file = File::open(filename).unwrap();
    let mut content = String::new();
    let mut buf_reader = BufReader::new(file);
    
    buf_reader.read_to_string(&mut content).unwrap();

    if ! key_file_is_well_form(&content) {
        return Err("The key isn't well form");
    }
    
    let filtered_result: String = content.chars()
        .filter(|character| ! character.is_digit(2))
        .collect();

    return Ok(filtered_result);
}

pub fn read_file(filename: &str) -> String {
    let file = File::open(filename).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    return contents;
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
        .fold(true, |acc, _| acc == false);

    if ! all_element_are_set {
        return Err("The key desn't have a clear sequence of identity lines");
    }

    return Ok(secret_lines);
}

pub fn key_is_well_form(key_matrix: &[&str; 4]) -> bool {
    for bits_string in key_matrix {
        if bits_string.len() != 8 {
            return false;
        }

        let is_binary_string = bits_string.chars()
            .filter(|character| character.is_digit(2))
            .fold(true, |acc, _| acc == false);
        
        if ! is_binary_string {
            return false;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_file_test() {
        let test_value: String = read_file("./testing_file/file_with_text.txt");
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
    fn get_secret_key_index_test() {
        let matrix_test = [
            "11011000", 
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
        let matrix_test = [
            "11011000", 
            "11001011", 
            "chocolatine", 
            "10100000"
        ];

        get_secret_key_index(&matrix_test).unwrap();
    }

    #[test]
    #[should_panic]
    fn get_secret_key_index_no_secret_found() {
        let matrix_test = [
            "11011000", 
            "11001011", 
            "10001111", 
            "10100000"
        ];
        
        get_secret_key_index(&matrix_test).unwrap();
    }

    #[test]
    fn key_is_well_form_test() {
        let matrix_test = [
            "11011000", 
            "11001011", 
            "10001110", 
            "10100000"
        ];
        assert_eq!(key_is_well_form(&matrix_test), true);
    }

    #[test]
    fn key_is_well_form_not_good_number_of_bits() {
        let matrix_test = [
            "11011000", 
            "11001011", 
            "1000111",
            "10100000"
        ];
        assert_eq!(key_is_well_form(&matrix_test), false);
    }

    #[test]
    fn key_is_well_form_not_binary_string() {
        let matrix_test = [
            "11011000", 
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

    #[test]
    fn key_file_is_well_form_test() {
        let key_string = String::from("G4C=[11011000 11001011 10001110 10100000]");
        assert_eq!(key_file_is_well_form(&key_string), true);
    }

    #[test]
    fn key_file_is_well_form_bad_header() {
        let key_string = String::from("G3C=[10101010 10101010 10101010 10101010]");
        assert_eq!(key_file_is_well_form(&key_string), false);
    }

    #[test]
    fn key_file_is_well_form_missing_closure_character() {
        let key_string = String::from("G4C=[10101010 10101010 10101010 10101010");
        assert_eq!(key_file_is_well_form(&key_string), false);
    }

    #[test]
    fn key_file_is_well_form_bad_number_of_characters() {
        let key_string = String::from("C=[10101010 10101010 10101010 10101010]");
        assert_eq!(key_file_is_well_form(&key_string), false);
    }

    #[test]
    fn read_key_test() {
        let key_string = String::from("G4C=[11011000 11001011 10001110 10100000]");
        assert_eq!(read_file("./testing_file/well_form_key.key"), key_string);
    }

    #[test]
    #[should_panic]
    fn read_key_bad_header() {
        read_key("./testing_file/bad_header.key").unwrap();
    }

    #[test]
    #[should_panic]
    fn read_key_missing_closure_character() {
        read_key("./testing_file/missing_closure_character.key").unwrap();
    }

    #[test]
    #[should_panic]
    fn read_key_bad_number_of_characters() {
        read_key("./testing_file/bad_number_of_characters.key").unwrap();
    }
}