//! # Exercice 07 — Références pendantes (difficile, ERREUR DE COMPILATION)
//!
//! Le compilateur refuse ces fonctions car elles produiraient des
//! références vers des données déjà libérées (« dangling references »).
//!
//! Corrige chaque fonction. Tu as le droit de changer les **types de retour**,
//! mais pas les tests. N'ajoute pas d'annotations de durée de vie (`'a`) :
//! ce n'est pas la solution attendue ici (et c'est pour le chapitre 10).
//!
//! Pour chaque fonction, explique en commentaire *quand* la donnée est libérée.
//!
//! Lancer : `cargo test -p ownership --features ex07 ex07`

pub fn make_greeting() -> &String {
    let s = String::from("Bonjour");
    &s
}

pub fn trimmed_upper(s: &str) -> &str {
    let upper = s.trim().to_uppercase();
    &upper
}

pub fn from_inner_scope() -> String {
    let r;
    {
        let s = String::from("scoped");
        r = &s;
    }
    r.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets() {
        assert_eq!(make_greeting(), "Bonjour");
    }

    #[test]
    fn trims_and_uppercases() {
        assert_eq!(trimmed_upper("  rust  "), "RUST");
    }

    #[test]
    fn reads_scoped_value() {
        assert_eq!(from_inner_scope(), "scoped");
    }
}
