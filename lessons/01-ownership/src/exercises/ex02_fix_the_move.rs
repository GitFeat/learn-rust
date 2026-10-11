//! # Exercice 02 — Corriger les « move » (facile, ERREUR DE COMPILATION)
//!
//! Ce fichier ne compile pas volontairement. Lis les messages de `rustc`,
//! comprends *pourquoi* la valeur a été déplacée, puis corrige.
//!
//! - `two_owners` : corrige avec `.clone()`.
//! - `length_then_use` : corrige **sans** `.clone()` (change la signature
//!   de `consume` pour emprunter au lieu de prendre possession).
//! - `repeat_name` : corrige comme tu veux, mais explique-toi (en commentaire)
//!   pourquoi l'erreur parle d'une « previous iteration of loop ».
//!
//! Ne modifie pas les tests.
//!
//! Lancer : `cargo test -p ownership --features ex02 ex02`

pub fn two_owners() -> (String, String) {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    (s1, s2)
}

fn consume(s: &String) -> usize {
    s.len()
}

pub fn length_then_use() -> (usize, String) {
    let s = String::from("ownership");
    let len = consume(&s);
    (len, s)
}

// Erreur value moved here, in previous iteration of loop, car lorsque name est consumé elle n'existe plus, donc impossible d'itérer dessus
// Si la valeur avec la quelle on itère est clonnée alors on compile
pub fn repeat_name(name: String, times: usize) -> Vec<String> {
    let mut names = Vec::new();
    for _ in 0..times {
        names.push(name.clone());
    }
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_owners_hold_hello() {
        assert_eq!(two_owners(), (String::from("hello"), String::from("hello")));
    }

    #[test]
    fn length_is_computed_and_string_kept() {
        assert_eq!(length_then_use(), (9, String::from("ownership")));
    }

    #[test]
    fn name_is_repeated() {
        let names = repeat_name(String::from("Ferris"), 3);
        assert_eq!(names, vec!["Ferris", "Ferris", "Ferris"]);
    }
}
