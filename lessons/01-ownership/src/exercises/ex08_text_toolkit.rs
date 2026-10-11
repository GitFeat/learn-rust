//! # Exercice 08 — Boîte à outils texte (difficile, synthèse)
//!
//! Combine tout le chapitre : emprunts, `&mut`, slices.
//! Ici, les mots sont séparés par **n'importe quel blanc** (espaces multiples,
//! tabulations, retours à la ligne).
//!
//! 1. `words` : la liste des mots, sous forme de slices de `text`
//!    (aucune `String` créée).
//! 2. `longest_word` : le mot le plus long (le **premier** en cas d'égalité),
//!    `None` s'il n'y a aucun mot.
//! 3. `split_key_value` : découpe `"clé = valeur"` au **premier** `=` et
//!    renvoie les deux côtés sans blancs autour. Sans `=`, renvoie
//!    `(entrée_sans_blancs, "")`.
//! 4. `capitalize_words` : met la première lettre de chaque mot en majuscule,
//!    en place. Les mots du résultat sont séparés par un seul espace.
//!
//! Question : pourquoi `words` peut-elle renvoyer `Vec<&str>` sans annotation
//! de durée de vie, alors que la fonction crée un `Vec` local ?
//!
//! Lancer : `cargo test -p ownership ex08`

pub fn words(text: &str) -> Vec<&str> {
    todo!()
}

pub fn longest_word(text: &str) -> Option<&str> {
    todo!()
}

pub fn split_key_value(line: &str) -> (&str, &str) {
    todo!()
}

pub fn capitalize_words(text: &mut String) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn points_into(slice: &str, text: &str) -> bool {
        let start = text.as_ptr() as usize;
        let end = start + text.len();
        let ptr = slice.as_ptr() as usize;
        ptr >= start && ptr + slice.len() <= end
    }

    #[test]
    fn splits_words() {
        let text = String::from("  the  borrow\tchecker\nis   your friend ");
        let result = words(&text);
        assert_eq!(
            result,
            vec!["the", "borrow", "checker", "is", "your", "friend"]
        );
        assert!(result.iter().all(|w| points_into(w, &text)));
        assert!(words("   ").is_empty());
    }

    #[test]
    fn finds_longest_word() {
        let text = String::from("ownership and borrowing rule");
        let longest = longest_word(&text);
        assert_eq!(longest, Some("ownership"));
        assert!(points_into(longest.unwrap(), &text));
        assert_eq!(longest_word("ab cd"), Some("ab"));
        assert_eq!(longest_word(""), None);
    }

    #[test]
    fn splits_key_value() {
        assert_eq!(split_key_value("name = Thibault"), ("name", "Thibault"));
        assert_eq!(split_key_value("expr=a=b"), ("expr", "a=b"));
        assert_eq!(split_key_value("  flag  "), ("flag", ""));

        let line = String::from(" lang = rust ");
        let (key, value) = split_key_value(&line);
        assert!(points_into(key, &line) && points_into(value, &line));
    }

    #[test]
    fn capitalizes_in_place() {
        let mut text = String::from("le  livre de\trust");
        capitalize_words(&mut text);
        assert_eq!(text, "Le Livre De Rust");

        let mut empty = String::new();
        capitalize_words(&mut empty);
        assert_eq!(empty, "");
    }
}
