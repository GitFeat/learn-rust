# Leçon 01 — Ownership (possession), emprunts et slices

> Référence : [Rust Book, chapitre 4 — Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
> (version française communautaire : [jimskapt.github.io/rust-book-fr](https://jimskapt.github.io/rust-book-fr/ch04-00-understanding-ownership.html))

L'ownership est *le* concept qui distingue Rust : il garantit la sécurité mémoire
**sans ramasse-miettes**, et toutes les vérifications ont lieu à la compilation.

---

## 1. La pile (stack) et le tas (heap)

📖 [4.1 — What is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap)

| Pile (stack) | Tas (heap) |
| --- | --- |
| Taille connue à la compilation (`i32`, `bool`, `char`, `[i32; 3]`…) | Taille inconnue ou variable (`String`, `Vec<T>`…) |
| Empiler / dépiler : très rapide | Allocation : il faut trouver de la place, plus lent |
| Libérée automatiquement en fin de fonction | Doit être libérée **une seule fois** — c'est le rôle de l'ownership |

Une `String` est en deux parties : sur la pile, un triplet `(pointeur, longueur, capacité)` ;
sur le tas, les octets du texte.

## 2. Les trois règles de l'ownership

1. Chaque valeur a **un propriétaire** (une variable).
2. Il ne peut y avoir **qu'un seul propriétaire à la fois**.
3. Quand le propriétaire sort de la portée (`}`), la valeur est **libérée** (`drop`).

```rust
{
    let s = String::from("hello"); // s owns the heap data
    // ... use s
} // s goes out of scope: memory is freed here
```

## 3. Move, Copy, Clone

📖 [Variables and Data Interacting with Move](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move)

- **Move** (par défaut pour les types qui possèdent du tas) : l'affectation ou le passage à
  une fonction **transfère** la propriété. L'ancienne variable devient inutilisable.

  ```rust
  let s1 = String::from("hi");
  let s2 = s1;          // move: s1 is no longer valid
  // println!("{s1}");  // error[E0382]: borrow of moved value
  ```

- **Copy** : les types simples stockés entièrement sur la pile (`i32`, `f64`, `bool`, `char`,
  tuples de types `Copy`…) sont **copiés** bit à bit. L'original reste valide.

  ```rust
  let x = 5;
  let y = x;            // copy: x is still valid
  ```

- **Clone** : copie **profonde** explicite (y compris les données du tas). Coûteux, donc visible.

  ```rust
  let s1 = String::from("hi");
  let s2 = s1.clone();  // both are valid, two heap allocations
  ```

Passer une valeur à une fonction suit exactement les mêmes règles (move ou copy),
tout comme la renvoyer.

## 4. Références et emprunt (`&` et `&mut`)

📖 [4.2 — References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

Une **référence** permet d'utiliser une valeur **sans en prendre possession** : on l'*emprunte*.

```rust
fn calculate_length(s: &String) -> usize { s.len() } // borrows, does not drop

let s = String::from("hello");
let len = calculate_length(&s); // s is still valid afterwards
```

Pour modifier une valeur empruntée, il faut une **référence mutable** — et la variable
elle-même doit être `mut` :

```rust
fn change(s: &mut String) { s.push_str(", world"); }

let mut s = String::from("hello");
change(&mut s);
```

### Les règles d'emprunt

À tout instant, pour une valeur donnée, on peut avoir **soit** :

- **une seule** référence mutable `&mut T`,
- **soit** un nombre quelconque de références immuables `&T`,

…mais **jamais les deux en même temps**. Et une référence doit **toujours être valide**
(pas de référence pendante / *dangling reference*).

La « durée de vie » d'une référence va de sa création à sa **dernière utilisation**
(*Non-Lexical Lifetimes*), pas forcément jusqu'à la fin du bloc :

```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;
println!("{r1} {r2}"); // last use of r1 and r2
let r3 = &mut s;       // OK: r1 and r2 are no longer used
```

Ces règles empêchent les *data races* dès la compilation.

## 5. Les slices

📖 [4.3 — The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)

Une **slice** est une référence vers une **portion contiguë** d'une collection. Elle ne
possède rien : c'est un `(pointeur, longueur)`.

```rust
let s = String::from("hello world");
let hello: &str = &s[0..5];   // or &s[..5]
let world: &str = &s[6..];    // up to the end
let all: &str = &s[..];

let a = [1, 2, 3, 4, 5];
let part: &[i32] = &a[1..3];  // [2, 3]
```

- Les littéraux `"..."` sont des `&str` : des slices vers le binaire.
- Préférer `&str` à `&String` en paramètre : la fonction accepte alors les deux
  (grâce à la *deref coercion*).
- Les indices de slice de `&str` sont en **octets** : couper au milieu d'un caractère UTF-8
  provoque un `panic!`.
- Tant qu'une slice existe, la collection d'origine est empruntée : impossible de la
  modifier (`clear()`, `push`…) pendant ce temps.

---

## Exercices

Dans `src/exercises/`, à faire **dans l'ordre**. Les consignes sont en tête de chaque fichier.

| # | Fichier | Thème | Difficulté | Type |
| --- | --- | --- | --- | --- |
| 01 | `ex01_string_ownership.rs` | `String`, `Copy` | ⭐ | `todo!()` |
| 02 | `ex02_fix_the_move.rs` | move, `clone` | ⭐ | erreur de compilation (`--features ex02`) |
| 03 | `ex03_borrow_instead_of_move.rs` | rendre vs emprunter | ⭐⭐ | `todo!()` |
| 04 | `ex04_mutable_references.rs` | `&mut` | ⭐⭐ | `todo!()` |
| 05 | `ex05_borrowing_rules.rs` | règles d'emprunt | ⭐⭐⭐ | erreur de compilation (`--features ex05`) |
| 06 | `ex06_string_slices.rs` | `&str`, `&[T]` | ⭐⭐⭐ | `todo!()` |
| 07 | `ex07_dangling_references.rs` | références pendantes | ⭐⭐⭐⭐ | erreur de compilation (`--features ex07`) |
| 08 | `ex08_text_toolkit.rs` | synthèse | ⭐⭐⭐⭐ | `todo!()` |

```bash
cargo test -p ownership              # every todo!() exercise (all fail at first)
cargo test -p ownership ex03         # a single exercise
cargo test -p ownership --features ex02 ex02   # an intentionally broken exercise
cargo test -p ownership --features broken      # everything, once all are fixed
```

Conseil : **lis les messages d'erreur du compilateur en entier**. Ils indiquent souvent
la ligne du move, celle de l'emprunt, et proposent une correction.

---

## Quiz de compréhension

Réponds par écrit (dans un fichier `QUIZ-REPONSES.md` à côté de celui-ci, par exemple).

1. Pourquoi `let y = x;` invalide `x` si `x` est une `String`, mais pas si c'est un `i32` ?
2. Que se passerait-il en mémoire si Rust autorisait deux propriétaires d'une même `String`
   et que les deux sortaient de portée ?
3. Quelle est la différence entre un *move* et un *clone* d'une `String` du point de vue
   de la pile et du tas ?
4. Un tuple `(i32, String)` est-il `Copy` ? Et `(i32, bool)` ? Pourquoi ?
5. Après `fn takes(s: String) {}` et `takes(s);`, peut-on encore utiliser `s` ? Comment
   écrire `takes` pour que ce soit possible ?
6. Énonce les deux règles d'emprunt. Quel type de bug empêchent-elles ?
7. Pourquoi ce code compile-t-il alors qu'on crée une `&mut` après deux `&` ?
   ```rust
   let mut s = String::from("a");
   let r1 = &s;
   println!("{r1}");
   let r2 = &mut s;
   r2.push('b');
   ```
8. Pourquoi une fonction ne peut-elle pas renvoyer `&String` vers une `String` créée à
   l'intérieur d'elle-même ? Quelle est la correction habituelle ?
9. Quelle est la différence entre `String`, `&String` et `&str` ? Pourquoi préférer `&str`
   en paramètre ?
10. Que se passe-t-il à l'exécution avec `&"héllo"[0..2]` ? Pourquoi ?
11. Pourquoi ne peut-on pas appeler `s.clear()` tant qu'une slice `&s[..3]` est encore utilisée ?
12. Que contient concrètement (sur la pile) une variable de type `&[i32]` ?
