// src/main.rs

#[allow(dead_code)]
/// Restituisce la stringa più lunga tra due.
/// Se hanno la stessa lunghezza, restituisce la prima.
/// 
/// TODO: Aggiungi i lifetime.  Domanda: servono due lifetime diversi o uno solo?
///                             RISPOSTA MIRKO: Ne basta uno solo
///                             CORREZIONE: No, perché il secondo parametro non vive
///                             abbastanza a lungo in questo modo.
///                             CORREZIONE 2: L'errore è voluto.
///                             Il compilatore non sa se longer restituirà a o b.
///                             Potrebbe restituire s2. Quindi deve assumere il caso peggiore.
///                             Era quindi giusto dire che ne basta uno solo, in questo modo
///                             Rust VIETA un bug (dangling reference) che invece sarebbe stato
///                             permesso in C++.
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

#[allow(dead_code)]
/// Restituisce la prima stringa, ignorando la seconda.
/// 
/// TODO: Aggiungi i lifetime.  Domanda: servono due lifetime diversi o uno solo?
///                             RISPOSTA MIRKO: No, ritorna solo a
fn first<'a>(a: &'a str, _b: &str) -> &'a str {
    a
}

#[allow(dead_code)]
/// Una struct che tiene due riferimenti, potenzialmente con lifetime diversi.
/// 
/// TODO:   Quanti lifetime parameters servono?
///         RISPOSTA MIRKO: Probabilmente 2 diversi
struct Pair<'a, 'b> {
    left: &'a str,
    right: &'b str,
}

#[allow(dead_code)]
impl<'a, 'b> Pair<'a, 'b> {
    /// Crea una nuova coppia
    fn new(left: &'a str, right: &'b str) -> Pair<'a, 'b> {
        Pair { left, right }
    }
    
    /// Restituisce il più lungo tra left e right
    fn longer(&self) -> &str {
        if self.left.len() >= self.right.len() {
            self.left
        } else {
            self.right
        }
    }
}

fn main() {
    let s1 = String::from("corta");
    let result;
    
    {
        let s2 = String::from("più lunga");
        result = longer(&s1, &s2); // <-- Qui è ok; fuori dallo scope no.
        println!("Più lunga: {}", result);
    }
    
    // Decommenta questa riga. Compila? Perché?
    // RISPOSTA MIRKO: No, perché ho usato solo un lifetime per tutti i parametri
    // in input e quindi `&s2` non vive abbastanza a lungo.
    // println!("Risultato: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longer_first_is_longer() {
        let a = "lunghissima";
        let b = "corta";
        assert_eq!(longer(a, b), "lunghissima");
    }

    #[test]
    fn test_longer_second_is_longer() {
        let a = "hi";
        let b = "ciao";
        assert_eq!(longer(a, b), "ciao");
    }

    #[test]
    fn test_longer_equal_length() {
        let a = "ciao";
        let b = "mare";
        assert_eq!(longer(a, b), "ciao"); // restituisce la prima
    }

    #[test]
    fn test_first_ignores_second() {
        let a = "prima";
        let b = "seconda";
        assert_eq!(first(a, b), "prima");
    }

    #[test]
    fn test_pair_longer() {
        let p = Pair::new("ab", "abcdef");
        assert_eq!(p.longer(), "abcdef");
    }

    #[test]
    fn test_first_different_lifetimes() {
        let a = String::from("lunga vita");
        let result;
        {
            let b = String::from("corta vita");
            result = first(&a, &b);
        }
        // result è ancora valido perché dipende solo da 'a'
        assert_eq!(result, "lunga vita");
    }
}