use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::stdin;
use std::iter::{Enumerate, repeat};
use std::ops::Add;
use std::str::Chars;
use colored::{ColoredString, Colorize};

/// Amount of attempts a user has
/// to guess the secret word.
const ATTEMPT_COUNT: u8 = 6;

/// Game result: an empty tuple if
/// the game loop terminated with
/// a correct guess of a [Word];
/// a [GameLost] error - if otherwise.
pub type Result = std::result::Result<(), GameLost>;

/// Runs the game loop until
/// either the [Word] is guessed
/// or the number of attempts
/// reaches [ATTEMPT_COUNT].
pub fn start_game_loop(word: &Word) -> Result {

    let mut attempt_n = 0;
    loop {

        if attempt_n == ATTEMPT_COUNT {
            return Result::Err(GameLost::with_word(word));
        }

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        let guess = String::from(input.trim_end());

        if guess.len() != 5 {
            println!("You'll need 5 characters to make it work!");
            continue;
        }

        let result = word.try_match(&guess);

        if result.full_match() {
            println!(
                "{} {}",
                "You won!".green(),
                format!("You needed {} attempts", attempt_n).normal()
            );
            return Result::Ok(());
        } else {
            result.print_result_for(&guess);
            attempt_n = attempt_n.add(1);
        }

    }

}


/// A custom [Error] type that reports
/// that a game is lost.
#[derive(Debug)]
pub struct GameLost {
    secret: String
}

impl GameLost {
    /// Constructs a new error object from some secret.
    fn with_word(word: &Word) -> GameLost {
        GameLost {
            secret: word.reveal().clone()
        }
    }
}

impl Display for GameLost {

    /// Prints out the message telling
    /// that a game is lost :(.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = format!(
            "{} The word was '{}'",
            "You lost :(".red(),
            &self.secret
        );
        write!(f, "{}", str)
    }
}

impl Error for GameLost {}


/// **The word**, i.e. the secret word
/// generated from the dictionary at
/// the start of the game and which a
/// user is supposed to guess.
#[derive(Debug)]
pub struct Word {

    val: String,
    letters: HashMap<char, HashSet<u8>>

}

impl Word {

    /// Same as [Word::analyze], but accepts a `&str`.
    pub fn analyze_str(word: &str) -> Word {
        Word::analyze(String::from(word))
    }

    /// Constructs a [Word] object out of a string.
    ///
    /// # Panics
    /// * Will `panic!` if the string a [Word] is
    ///   supposed to be constructed from is not
    ///   precisely 5 characters long.
    pub fn analyze(word: String) -> Word {
        if word.len() != 5 {
            panic!("Secret word must be exactly 5 chars long, got {}", word)
        }

        let mut mm: HashMap<char, HashSet<u8>> = HashMap::new();

        for (ind, ch) in word.chars().enumerate() {
            if mm.contains_key(&ch) {
                let hset = mm
                    .get_mut(&ch)
                    .unwrap();
                hset.insert(ind as u8);
            } else {
                let mut hset = HashSet::new();
                hset.insert(ind as u8);
                mm.insert(ch, hset);
            }
        }

        Word {
            val: word,
            letters: mm
        }
    }

    /// Takes a string and checks it letter-by-letter
    /// against the internally contained secret, thus
    /// producing a [GuessResult].
    pub fn try_match(&self, word: &String) -> GuessResult {
        if self.val.eq(word) {
            return GuessResult::new_all_green();
        }

        let mut guess = GuessResult::new_empty();

        for (ind, char) in word.chars().enumerate() {
            let match_result = self.letters
                .get(&char)
                .map(|hset| hset.contains(&(ind as u8)))
                .map(|rs|
                    if rs {
                        MatchResult::Match
                    } else {
                        MatchResult::Exists
                    }
                )
                .unwrap_or(MatchResult::None);
            guess.push(match_result);
        }

        guess
    }

    /// Shows the secret word.
    fn reveal(&self) -> &String {
        &self.val
    }

}

/// An in-loop stateful object that
/// tracks letter matches.
#[derive(Debug)]
pub struct GuessResult {
    result: Vec<MatchResult>
}

impl GuessResult {

    /// Returns `true` if all letters
    /// and their positions have been
    /// guessed correctly.
    pub fn full_match(&self) -> bool {
        let res = &self.result.iter()
            .all(|item| *item == MatchResult::Match);
        *res
    }

    /// Pretty-prints the result of a guess attempt.
    pub fn print_result_for(&self, word: &String) {
        let mut chars = word.chars().enumerate();

        println!(
            "{} {} {} {} {}",
            GuessResult::get_and_colorize(&mut chars, &self),
            GuessResult::get_and_colorize(&mut chars, &self),
            GuessResult::get_and_colorize(&mut chars, &self),
            GuessResult::get_and_colorize(&mut chars, &self),
            GuessResult::get_and_colorize(&mut chars, &self)
        );
    }

    /// Takes next letter from the character
    /// iterator and colors it according to
    /// the [GuessResult].
    fn get_and_colorize(e: &mut Enumerate<Chars>, result: &GuessResult) -> ColoredString {
        let tuple = e.next().unwrap();
        let ch = tuple.1;
        let res = *result.result
            .get(tuple.0).unwrap();

        match res {
            MatchResult::Match => String::from(ch).green().bold(),
            MatchResult::Exists => String::from(ch).yellow().bold(),
            MatchResult::None => String::from(ch).normal()
        }
    }

    /// Creates a [GuessResult] that starts
    /// with all buckets filled with [MatchResult::Match]
    fn new_all_green() -> GuessResult {
        let result = Vec::from_iter(
            repeat(MatchResult::Match)
                .take(5)
        );
        GuessResult {
            result
        }
    }

    /// Creates an empty [GuessResult].
    fn new_empty() -> GuessResult {
        GuessResult { result: Vec::with_capacity(5) }
    }

    /// Tracks a new letter [MatchResult].
    fn push(&mut self, r: MatchResult) {
        self.result.push(r)
    }

}

/// Represents letter match result.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum MatchResult {

    /// The guessed letter is in the secret
    /// word at the correctly guessed position.
    Match,

    /// Letter has been guessed correctly,
    /// but its position hasn't.
    Exists,

    /// There is no such letter in the secret word.
    None

}






mod test {
    use crate::game::{GuessResult, MatchResult, Word};

    #[test]
    fn guess_result_full_match() {
        let result = vec![
            MatchResult::Match,
            MatchResult::Match,
            MatchResult::Match,
            MatchResult::Match,
            MatchResult::Match
        ];

        let guess = GuessResult { result };
        assert!(guess.full_match())
    }

    #[test]
    fn guess_result_new_all_green() {
        let guess = GuessResult::new_all_green();
        assert!(guess.full_match())
    }

    #[test]
    fn guess_result_new_empty() {
        let guess = GuessResult::new_empty();
        assert!(guess.result.is_empty())
    }

    #[test]
    #[should_panic]
    fn word_analyze_not_5_characters() {
        let word = String::from("bank");
        Word::analyze(word);
    }

    #[test]
    #[should_panic]
    fn word_analyze_str_not_5_characters() {
        let word = "bank";
        Word::analyze_str(word);
    }

    #[test]
    fn word_analyze_check_internal_word() {
        let word = Word::analyze_str("bathe");

        assert_eq!(
            String::from("bathe"),
            word.val
        )
    }

    #[test]
    fn word_analyze_all_letters_different() {
        let word = Word::analyze_str("bathe");

        assert_eq!(5, word.letters.len());

        assert!(word.letters.contains_key(&'b'));
        assert_eq!(
            1,
            word.letters
                .get(&'b').unwrap()
                .len()
        );
        assert!(word.letters
            .get(&'b').unwrap()
            .contains(&(0 as u8))
        );

        assert!(word.letters.contains_key(&'a'));
        assert_eq!(
            1,
            word.letters
                .get(&'a').unwrap()
                .len()
        );
        assert!(word.letters
            .get(&'a').unwrap()
            .contains(&(1 as u8))
        );

        assert!(word.letters.contains_key(&'t'));
        assert_eq!(
            1,
            word.letters
                .get(&'t').unwrap()
                .len()
        );
        assert!(word.letters
            .get(&'t').unwrap()
            .contains(&(2 as u8))
        );

        assert!(word.letters.contains_key(&'h'));
        assert_eq!(
            1,
            word.letters
                .get(&'h').unwrap()
                .len()
        );
        assert!(word.letters
            .get(&'h').unwrap()
            .contains(&(3 as u8))
        );

        assert!(word.letters.contains_key(&'e'));
        assert_eq!(
            1,
            word.letters
                .get(&'e').unwrap()
                .len()
        );
        assert!(word.letters
            .get(&'e').unwrap()
            .contains(&(4 as u8))
        );

    }

    #[test]
    fn word_analyze_all_letters_same() {
        let word = Word::analyze_str("aaaaa");

        assert_eq!(1, word.letters.len());
        assert!(word.letters.contains_key(&'a'));

        let mut as_sorted_vec: Vec<u8> = word.letters
            .get(&'a').unwrap()
            .into_iter()
            .map(|ptr|*ptr)
            .collect();
        as_sorted_vec.sort_unstable();

        assert_eq!(5, as_sorted_vec.len());
        assert_eq!(as_sorted_vec, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn word_try_match_full_match() {
        let word = Word::analyze_str("bathe");

        let guess = String::from("bathe");
        let result = word.try_match(&guess);
        assert!(result.full_match())
    }

    #[test]
    fn word_try_match_partial_match() {
        let word = Word::analyze_str("bathe");

        let guess = String::from("braid");
        let result = word.try_match(&guess);
        assert!(!result.full_match());

        assert_eq!(
            MatchResult::Match,
            *result.result.get(0).unwrap()
        );
        assert_eq!(
            MatchResult::None,
            *result.result.get(1).unwrap()
        );
        assert_eq!(
            MatchResult::Exists,
            *result.result.get(2).unwrap()
        );
        assert_eq!(
            MatchResult::None,
            *result.result.get(3).unwrap()
        );
        assert_eq!(
            MatchResult::None,
            *result.result.get(4).unwrap()
        );
    }

}

