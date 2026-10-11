//! # Exercice 06 — Slices `&str` et `&[T]` (moyen)
//!
//! Les mots sont séparés par un espace `' '` (comme dans le Rust Book).
//! Aucune fonction ne doit allouer : on renvoie des **morceaux** de l'entrée.
//!
//! 1. `first_word` : premier mot (la chaîne entière s'il n'y a pas d'espace).
//!    Essaie d'abord la version du livre avec `as_bytes()` + `enumerate()`.
//! 2. `last_word` : dernier mot.
//! 3. `sum` : somme d'une slice d'entiers.
//! 4. `middle` : la slice sans son premier ni son dernier élément
//!    (slice vide si moins de 3 éléments).
//!
//! Question : pourquoi `first_word` prend-elle `&str` plutôt que `&String` ?
//!
//! Lancer : `cargo test -p ownership ex06`

pub fn first_word(s: &str) -> &str {
    todo!()
}

pub fn last_word(s: &str) -> &str {
    todo!()
}

pub fn sum(numbers: &[i32]) -> i32 {
    todo!()
}

pub fn middle(numbers: &[i32]) -> &[i32] {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word(""), "");

        // Works with a `&String` too, thanks to deref coercion.
        let owned = String::from("borrow checker");
        assert_eq!(first_word(&owned), "borrow");
    }

    #[test]
    fn first_word_points_into_input() {
        let text = String::from("no allocation here");
        let word = first_word(&text);
        assert_eq!(word.as_ptr(), text.as_ptr());
    }

    #[test]
    fn finds_last_word() {
        assert_eq!(last_word("hello big world"), "world");
        assert_eq!(last_word("single"), "single");
        assert_eq!(last_word(""), "");
    }

    #[test]
    fn sums_slices() {
        let numbers = [1, 2, 3, 4, 5];
        assert_eq!(sum(&numbers), 15);
        assert_eq!(sum(&numbers[1..3]), 5);
        assert_eq!(sum(&[]), 0);
    }

    #[test]
    fn takes_middle() {
        assert_eq!(middle(&[1, 2, 3, 4, 5]), &[2, 3, 4]);
        assert_eq!(middle(&[1, 2, 3]), &[2]);
        assert_eq!(middle(&[1, 2]), &[] as &[i32]);
        assert_eq!(middle(&[]), &[] as &[i32]);
    }
}
