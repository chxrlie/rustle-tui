use rand::prelude::*;

const ADJECTIVES: &str = include_str!("adjectives.txt");
const NOUNS: &str = include_str!("nouns.txt");
const SYMBOLS: &[char] = &['!', '@', '#', '$', '%', '^', '&', '*', '-', '=', '_', '+', '£'];

pub struct PasswordGenerator {
    adjectives: Vec<&'static str>,
    nouns: Vec<&'static str>,
}

impl PasswordGenerator {
    pub fn new() -> Self {
        let adjectives: Vec<&'static str> = ADJECTIVES.lines().collect();
        let nouns: Vec<&'static str> = NOUNS.lines().collect();
        Self { adjectives, nouns }
    }

    pub fn generate(&self) -> String {
        let mut rng = rand::rng();

        let adjective = *self.adjectives.choose(&mut rng).unwrap_or(&"");
        let noun = *self.nouns.choose(&mut rng).unwrap_or(&"");
        let symbol1 = *SYMBOLS.choose(&mut rng).unwrap_or(&'!');
        let symbol2 = *SYMBOLS.choose(&mut rng).unwrap_or(&'!');
        let number = rng.random_range(0..=9999);

        format!("{}{}{}{}{}", adjective, symbol1, noun, symbol2, number)
    }
}