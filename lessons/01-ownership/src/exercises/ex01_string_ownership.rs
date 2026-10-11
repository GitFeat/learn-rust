//! # Exercice 01 — Posséder une `String` (très facile)
//!
//! Remplace chaque `todo!()` par une implémentation.
//!
//! 1. `create_owned_string` : renvoie une `String` possédée contenant `"rust"`.
//! 2. `build_greeting` : prend possession de `name` et renvoie `"Bonjour, <name> !"`.
//!    Contrainte : utilise `push_str` (pas de `format!`).
//! 3. `duplicate_number` : renvoie `(x, x)`. Pourquoi est-ce que ça compile
//!    alors qu'on « utilise » `x` deux fois ? (indice : le trait `Copy`)
//!
//! Lancer : `cargo test -p ownership ex01`

// On construit la chaine puis on return
pub fn create_owned_string() -> String {
    let s: String = String::from("rust");
    return s
}

// On construit la chaine avec push_str(), et on emprute name avec &
pub fn build_greeting(name: String) -> String {
    let mut hello: String = String::from("Bonjour, ");
    hello.push_str(&name);
    hello.push_str(" !");
    hello
}

// Compile car i32 implémente le trait Copy, donc x dans le return est copié 2x
pub fn duplicate_number(x: i32) -> (i32, i32) {
    return (x, x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_owned_string() {
        let s: String = create_owned_string();
        assert_eq!(s, "rust");
    }

    #[test]
    fn builds_greeting() {
        let name = String::from("Thibault");
        assert_eq!(build_greeting(name), "Bonjour, Thibault !");
    }

    #[test]
    fn duplicates_copy_value() {
        assert_eq!(duplicate_number(42), (42, 42));
    }
}
