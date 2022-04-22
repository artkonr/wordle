use std::collections::{HashMap, HashSet};
use std::iter::{Enumerate, repeat};
use std::str::Chars;
use colored::{ColoredString, Colorize};

#[derive(Debug)]
pub struct Word {

    val: String,
    letters: HashMap<char, HashSet<u8>>

}

impl Word {

    pub fn analyze_str(word: &str) -> Word {
        Word::analyze(String::from(word))
    }

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
                        MatchResult::MATCH
                    } else {
                        MatchResult::EXISTS
                    }
                )
                .unwrap_or(MatchResult::NONE);
            guess.push(match_result);
        }

        guess
    }

    pub fn reveal(&self) -> &String {
        &self.val
    }

}

#[derive(Debug)]
pub struct GuessResult {
    result: Vec<MatchResult>
}

impl GuessResult {

    pub fn full_match(&self) -> bool {
        let res = &self.result.iter()
            .all(|item| *item == MatchResult::MATCH);
        *res
    }

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

    fn get_and_colorize(e: &mut Enumerate<Chars>, result: &GuessResult) -> ColoredString {
        let tuple = e.next().unwrap();
        let ch = tuple.1;
        let res = *result.result
            .get(tuple.0).unwrap();

        match res {
            MatchResult::MATCH => String::from(ch).green().bold(),
            MatchResult::EXISTS => String::from(ch).yellow().bold(),
            MatchResult::NONE => String::from(ch).normal()
        }
    }

    fn new_all_green() -> GuessResult {
        let result = Vec::from_iter(
            repeat(MatchResult::MATCH)
                .take(5)
        );
        GuessResult {
            result
        }
    }

    fn new_empty() -> GuessResult {
        GuessResult { result: Vec::with_capacity(5) }
    }

    fn push(&mut self, r: MatchResult) {
        self.result.push(r)
    }

}


#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum MatchResult {
    MATCH,
    EXISTS,
    NONE
}



mod test {
    use crate::rules::{GuessResult, MatchResult, Word};

    #[test]
    fn guess_result_full_match() {
        let result = vec![
            MatchResult::MATCH,
            MatchResult::MATCH,
            MatchResult::MATCH,
            MatchResult::MATCH,
            MatchResult::MATCH
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
            MatchResult::MATCH,
            *result.result.get(0).unwrap()
        );
        assert_eq!(
            MatchResult::NONE,
            *result.result.get(1).unwrap()
        );
        assert_eq!(
            MatchResult::EXISTS,
            *result.result.get(2).unwrap()
        );
        assert_eq!(
            MatchResult::NONE,
            *result.result.get(3).unwrap()
        );
        assert_eq!(
            MatchResult::NONE,
            *result.result.get(4).unwrap()
        );
    }

}

