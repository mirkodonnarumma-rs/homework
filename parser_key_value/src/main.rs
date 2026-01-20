// src/main.rs

/// Una coppia chiave-valore che contiene riferimenti a una stringa esistente.
/// 
/// TODO: Aggiungi il lifetime parameter corretto
struct KeyValue<'a> {
    key: &'a str,
    value: &'a str,
}

/// Parsa una stringa nel formato "chiave=valore".
/// Restituisce None se non trova '='.
/// 
/// TODO: Aggiungi i lifetime corretti alla firma
fn parse<'a>(input: &'a str) -> Option<KeyValue<'a>> {
    let pos = input.find('=')?;
    let key = &input[..pos];
    let value = &input[pos + 1..];
    Some(KeyValue { key, value })
}

fn main() {
    let input = String::from("nome=Mario");
    
    if let Some(kv) = parse(&input) {
        println!("Chiave: {}", kv.key);
        println!("Valore: {}", kv.value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid() {
        let input = "nome=Mario";
        let kv = parse(input).unwrap();
        assert_eq!(kv.key, "nome");
        assert_eq!(kv.value, "Mario");
    }

    #[test]
    fn test_parse_empty_value() {
        let input = "chiave=";
        let kv = parse(input).unwrap();
        assert_eq!(kv.key, "chiave");
        assert_eq!(kv.value, "");
    }

    #[test]
    fn test_parse_no_equals() {
        let input = "invalid";
        assert!(parse(input).is_none());
    }

    #[test]
    fn test_parse_multiple_equals() {
        // "a=b=c" dovrebbe dare key="a", value="b=c"
        let input = "a=b=c";
        let kv = parse(input).unwrap();
        assert_eq!(kv.key, "a");
        assert_eq!(kv.value, "b=c");
    }
}