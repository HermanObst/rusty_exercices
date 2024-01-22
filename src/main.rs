use std::fs::File;
use std::io::Read;
fn main() {
    println!("Hello, world!");
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