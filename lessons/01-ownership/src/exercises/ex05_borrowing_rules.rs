//! # Exercice 05 — Les règles d'emprunt (moyen, ERREUR DE COMPILATION)
//!
//! Chaque fonction viole une règle d'emprunt. Corrige-les en
//! **réorganisant le code** (ordre des instructions, portées, copie d'un
//! entier…). Contrainte : `.clone()` est **interdit** dans ce fichier.
//!
//! Pour chaque fonction, écris en commentaire la règle violée :
//! « une seule `&mut` à la fois » ou « pas de `&mut` pendant qu'un `&` est vivant ».
//!
//! Ne modifie pas les tests.
//!
//! Lancer : `cargo test -p ownership --features ex05 ex05`

//une seule &mut à la fois !
pub fn two_mutable_borrows() -> String {
    let mut s = String::from("hello");
    let r1 = &mut s;
    r1.push_str(", world");
    let r2 = &mut s;
    r2.push('!');
    s
}

/// Must return the length *before* the modification, and the modified string.
/// pas de `&mut` pendant qu'un `&` est vivant
pub fn read_while_writing() -> (usize, String) {
    let mut s = String::from("borrow");
    let before = s.len();
    s.push_str("ing");
    (before, s)
}

/// Must return the first element and the vector after the push.
/// pas de `&mut` pendant qu'un `&` est vivant
pub fn first_then_push() -> (i32, Vec<i32>) {
    let mut v = vec![1, 2, 3];
    let first = v[0];
    v.push(4);
    (first, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutates_twice() {
        assert_eq!(two_mutable_borrows(), "hello, world!");
    }

    #[test]
    fn reads_then_writes() {
        assert_eq!(read_while_writing(), (6, String::from("borrowing")));
    }

    #[test]
    fn keeps_first_element() {
        assert_eq!(first_then_push(), (1, vec![1, 2, 3, 4]));
    }
}
