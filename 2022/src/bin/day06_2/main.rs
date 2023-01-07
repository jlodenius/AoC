use std::fs;

struct SignalHandler {
    characters: Vec<char>,
}

impl SignalHandler {
    fn add_character(&mut self, char_to_add: char) {
        if self.characters.len() == 14 {
            self.characters.push(char_to_add);
            self.characters.remove(0);
        } else {
            self.characters.push(char_to_add);
        }
    }
    fn is_unique_sequence(&self) -> bool {
        if &self.characters.len() < &14 {
            return false;
        }
        let clone = &self.characters.to_vec();
        self.characters.iter().all(|x| {
            clone
                .into_iter()
                .filter(|&y| y == x)
                .collect::<Vec<&char>>()
                .len()
                < 2
        })
    }
}

fn main() {
    let file_path = "day6_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut handler = SignalHandler {
        characters: Vec::new(),
    };
    for (index, character) in contents.chars().enumerate() {
        handler.add_character(character);
        if handler.is_unique_sequence() {
            println!("CHARACTER -> {}", character);
            println!("RESULT -> {}", index + 1);
            break;
        }
    }
}
