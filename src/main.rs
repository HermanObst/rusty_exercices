use std::fs::File;
use std::io::Read;
fn main() {
    println!("Hello, world!");
    let test = "HolaBaby".to_string();
    main_loop(test, 5);
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

fn print_console_display(word: String, reconstructed_word: String) -> () {
    todo!()
}

fn main_loop(word_vec: String, max_attempts: u8) -> () {
    let mut attempts: u8 = 0;
    
    while attempts < max_attempts {
        let mut letter = String::new();
        std::io::stdin()
            .read_line(&mut letter)
            .expect("Error leyendo la linea."); 
        letter.pop();
        
        if word_vec.contains(&letter) {
            println!("DALEE");
        } else {
            attempts+= 1;
        }
    }
    todo!()
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