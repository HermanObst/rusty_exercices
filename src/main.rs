use std::fs::{read, File};
use std::io::Read;
fn main() {
    let file_string = read_file("src/palabras.txt").unwrap();
    let words = process_file(file_string);
    let first_word = words.first().cloned().unwrap();
    main_loop(first_word, 5);
    println!("TERMINO");
}

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn process_file(string_file: String) -> Vec<String> {
    let mut words = Vec::new();
    for word in string_file.split('\n') {
        words.push(word.to_string());
    }
    words
}

// fn print_console_display(word: String, reconstructed_word: String) -> () {
//     todo!()
// }

fn letter_index(word_string: String, letter: String) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();
    let char = letter.chars().next().unwrap();

    for (index, ch) in word_string.char_indices() {
        if ch == char {
            indices.push(index);
        }
    }
    indices
}

fn fill_word_by_indices(filling_word: &mut Vec<u8>, indices: Vec<usize>, letter: String) {
    let letter = letter.bytes().next().unwrap();
    for index in indices {
        filling_word[index] = letter
    }
}

fn main_loop(word_string: String, max_attempts: u8) {
    let mut attempts: u8 = 0;
    let mut filling_word: Vec<u8> = vec![0; word_string.len()];

    while attempts < max_attempts {
        let mut letter = String::new();
        std::io::stdin()
            .read_line(&mut letter)
            .expect("Error leyendo la linea.");
        letter.pop();

        let indices = letter_index(word_string.clone(), letter.clone());

        if indices.is_empty() {
            attempts += 1;
        } else {
            fill_word_by_indices(&mut filling_word, indices, letter);
        }
        println!("DATA! {:?}", filling_word);
    }
}

#[test]
fn test_file_reading() {
    let result = read_file("src/palabras.txt").unwrap();
    assert_eq!(result, "Hola\nComo\nestas?");
}

#[test]
fn test_process_file() {
    let file_string = read_file("src/palabras.txt").unwrap();
    let words = process_file(file_string);
    let expected_output = vec!["Hola", "Como", "estas?"];
    assert_eq!(words, expected_output);
}

#[test]
fn test_letter_index() {
    let word = "Hello".to_string();
    let char1 = "e".to_string();
    let char2 = "l".to_string();
    let char0 = "x".to_string();

    assert_eq!([1].to_vec(), letter_index(word.clone(), char1));
    assert_eq!([2, 3].to_vec(), letter_index(word.clone(), char2));
    assert_eq!(Vec::<usize>::new(), letter_index(word, char0));
}

#[test]
fn test_fill_word_by_indices() {
    let word_string = "Hello".to_string();
    let mut filling_word: Vec<u8> = vec![0; word_string.len()];
    let char_string = "l".to_string();
    let indices = letter_index(word_string, char_string.clone());
    let char_result = char_string.chars().next().unwrap() as u8;

    fill_word_by_indices(&mut filling_word, indices, char_string);

    assert_eq!(filling_word, [0, 0, char_result, char_result, 0].to_vec());
}
