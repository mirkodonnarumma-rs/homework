/// Trova la parola più lunga in un testo.
/// Restituisce None se il testo è vuoto o non contiene parole.
/// 
/// TODO: Serve specificare il lifetime? Prova senza, poi rifletti.
fn longest_word(text: &str) -> Option<&str> {
    let iter = text.split_whitespace();
    Some(iter.max_by_key(|word| word.len()))?
}

/// Trova la parola più corta in un testo.
/// Restituisce None se il testo è vuoto o non contiene parole.
fn shortest_word(text: &str) -> Option<&str> {
    let iter = text.split_whitespace();
    Some(iter.min_by_key(|word| word.len()))?
}

fn main() {
    let frase = String::from("Rust è un linguaggio fantastico");
    
    if let Some(lunga) = longest_word(&frase) {
        println!("Parola più lunga: {}", lunga);
    }
    
    if let Some(corta) = shortest_word(&frase) {
        println!("Parola più corta: {}", corta);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test per longest_word
    #[test]
    fn test_longest_basic() {
        let text = "ciao mondo bellissimo";
        assert_eq!(longest_word(text), Some("bellissimo"));
    }

    #[test]
    fn test_longest_first_word() {
        let text = "straordinario ma bello";
        assert_eq!(longest_word(text), Some("straordinario"));
    }

    #[test]
    fn test_longest_empty() {
        let text = "";
        assert_eq!(longest_word(text), None);
    }

    #[test]
    fn test_longest_single_word() {
        let text = "ciao";
        assert_eq!(longest_word(text), Some("ciao"));
    }

    // Test per shortest_word
    #[test]
    fn test_shortest_basic() {
        let text = "ciao mondo io";
        assert_eq!(shortest_word(text), Some("io"));
    }

    #[test]
    fn test_shortest_empty() {
        let text = "";
        assert_eq!(shortest_word(text), None);
    }

    #[test]
    fn test_shortest_only_spaces() {
        let text = "   ";
        assert_eq!(shortest_word(text), None);
    }

    // Test che verifica che restituiamo RIFERIMENTI, non copie
    #[test]
    fn test_returns_reference_not_copy() {
        let text = "trova questa parola";
        let result = longest_word(text).unwrap();
        
        // Verifica che result punti DENTRO text
        let text_start = text.as_ptr() as usize;
        let text_end = text_start + text.len();
        let result_start = result.as_ptr() as usize;
        
        assert!(result_start >= text_start && result_start < text_end);
    }
}