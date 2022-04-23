use rand::Rng;
use crate::game::Word;

pub trait Dictionary {

    fn generate(&self) -> Word;

}


pub struct StaticDict;

impl Dictionary for StaticDict {

    fn generate(&self) -> Word {
        let mut rnd = rand::thread_rng();

        let file_n = rnd.gen_range(1..7);

        let file_contents = match file_n {
            1 => include_str!("../assets/part-1.csv"),
            2 => include_str!("../assets/part-2.csv"),
            3 => include_str!("../assets/part-3.csv"),
            4 => include_str!("../assets/part-4.csv"),
            5 => include_str!("../assets/part-5.csv"),
            6 => include_str!("../assets/part-6.csv"),
            _ => panic!("Weird file index")
        };

        let lines: Vec<&str> = file_contents
            .split('\n')
            .collect();

        let line_n = rnd.gen_range(0..lines.len());
        let word = lines.get(line_n)
            .expect("Failed to get word");

        Word::analyze_str(word)
    }

}

