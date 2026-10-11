//! # Exercice 03 — Rendre la propriété vs emprunter (facile)
//!
//! 1. `take_and_give_back` : prend possession de `s`, puis la **rend** avec
//!    sa longueur (version « lourde », sans références).
//! 2. `calculate_length` : même chose, mais en **empruntant** `s`.
//! 3. `count_vowels` : compte les voyelles (`a e i o u y`, majuscules
//!    comprises) sans prendre possession de la chaîne.
//!
//! Question : dans les tests, pourquoi `text` est-il encore utilisable après
//! l'appel à `calculate_length`, mais pas après `take_and_give_back` ?
//!
//! Lancer : `cargo test -p ownership ex03`

pub fn take_and_give_back(s: String) -> (String, usize) {
    todo!()
}

pub fn calculate_length(s: &String) -> usize {
    todo!()
}

pub fn count_vowels(s: &String) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gives_ownership_back() {
        let text = String::from("hello");
        let (text, len) = take_and_give_back(text);
        assert_eq!(len, 5);
        assert_eq!(text, "hello");
    }

    #[test]
    fn borrows_to_compute_length() {
        let text = String::from("borrowing");
        let len = calculate_length(&text);
        assert_eq!(len, 9);
        // `text` is still owned here.
        assert_eq!(text, "borrowing");
    }

    #[test]
    fn counts_vowels_without_moving() {
        let text = String::from("Rust Is Easy");
        assert_eq!(count_vowels(&text), 5);
        assert_eq!(count_vowels(&String::new()), 0);
        assert_eq!(text, "Rust Is Easy");
    }
}
