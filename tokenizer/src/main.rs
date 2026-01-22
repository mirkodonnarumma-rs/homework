// src/main.rs

/// Un tokenizer che splitta una stringa per spazi.
/// Mantiene traccia di quanto testo resta da processare.
/// 
/// TODO: Aggiungi il lifetime parameter
struct Tokenizer<'a> {
    remaining: &'a str,  // TODO: lifetime
}

impl<'a> Tokenizer<'a> {
    /// Crea un nuovo Tokenizer dal testo dato.
    /// 
    /// TODO: Aggiungi i lifetime corretti
    pub fn new(text: &'a str) -> Tokenizer<'a> {
        Tokenizer { remaining: text }
    }
}

/// TODO: Implementa Iterator per Tokenizer
/// 
/// Suggerimento: il tipo Item deve essere &str con il lifetime giusto
impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a str; // TODO: cambia questo tipo
    
    fn next(&mut self) -> Option<Self::Item> {
        self.remaining = self.remaining.trim_start();
        if self.remaining.is_empty() {
            return None;
        }
        let end = match self.remaining.find(' ') {
            Some(pos) => pos,
            None => self.remaining.len(),
        };
        let token = &self.remaining[..end];
        self.remaining = &self.remaining[end..];

        Some(token)

    }
}

fn main() {
    let testo = String::from("Rust è fantastico");
    let tokenizer = Tokenizer::new(&testo);
    
    for token in tokenizer {
        println!("Token: '{}'", token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let text = "ciao mondo";
        let tokens: Vec<&str> = Tokenizer::new(text).collect();
        assert_eq!(tokens, vec!["ciao", "mondo"]);
    }

    #[test]
    fn test_multiple_spaces() {
        let text = "uno   due    tre";
        let tokens: Vec<&str> = Tokenizer::new(text).collect();
        assert_eq!(tokens, vec!["uno", "due", "tre"]);
    }

    #[test]
    fn test_empty_string() {
        let text = "";
        let tokens: Vec<&str> = Tokenizer::new(text).collect();
        assert_eq!(tokens, Vec::<&str>::new());
    }

    #[test]
    fn test_only_spaces() {
        let text = "    ";
        let tokens: Vec<&str> = Tokenizer::new(text).collect();
        assert_eq!(tokens, Vec::<&str>::new());
    }

    #[test]
    fn test_single_word() {
        let text = "ciao";
        let tokens: Vec<&str> = Tokenizer::new(text).collect();
        assert_eq!(tokens, vec!["ciao"]);
    }

    #[test]
    fn test_leading_trailing_spaces() {
        let text = "  ciao mondo  ";
        let tokens: Vec<&str> = Tokenizer::new(text).collect();
        assert_eq!(tokens, vec!["ciao", "mondo"]);
    }

    #[test]
    fn test_iterator_step_by_step() {
        let text = "a b c";
        let mut tokenizer = Tokenizer::new(text);
        
        assert_eq!(tokenizer.next(), Some("a"));
        assert_eq!(tokenizer.next(), Some("b"));
        assert_eq!(tokenizer.next(), Some("c"));
        assert_eq!(tokenizer.next(), None);
    }
}