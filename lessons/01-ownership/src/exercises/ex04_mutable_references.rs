//! # Exercice 04 — Références mutables `&mut` (moyen)
//!
//! Toutes ces fonctions modifient leur argument **en place** et ne renvoient rien.
//!
//! 1. `add_exclamation` : ajoute `!` à la fin.
//! 2. `shout` : met la chaîne en majuscules puis ajoute `!`.
//!    Indice : on peut remplacer toute la valeur pointée avec `*s = ...`.
//! 3. `swap_strings` : échange le contenu de `a` et `b`.
//!    Indice : regarde `std::mem::swap`.
//! 4. `increment_all` : ajoute 1 à chaque élément.
//!    Indice : `iter_mut()` et le déréférencement `*n`.
//!
//! Lancer : `cargo test -p ownership ex04`

pub fn add_exclamation(s: &mut String) {
    todo!()
}

pub fn shout(s: &mut String) {
    todo!()
}

pub fn swap_strings(a: &mut String, b: &mut String) {
    todo!()
}

pub fn increment_all(numbers: &mut Vec<i32>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_exclamation() {
        let mut s = String::from("hello");
        add_exclamation(&mut s);
        assert_eq!(s, "hello!");
    }

    #[test]
    fn shouts() {
        let mut s = String::from("ownership");
        shout(&mut s);
        assert_eq!(s, "OWNERSHIP!");
    }

    #[test]
    fn swaps() {
        let mut a = String::from("left");
        let mut b = String::from("right");
        swap_strings(&mut a, &mut b);
        assert_eq!((a.as_str(), b.as_str()), ("right", "left"));
    }

    #[test]
    fn increments() {
        let mut numbers = vec![1, 2, 3];
        increment_all(&mut numbers);
        assert_eq!(numbers, vec![2, 3, 4]);

        let mut empty: Vec<i32> = Vec::new();
        increment_all(&mut empty);
        assert!(empty.is_empty());
    }
}
