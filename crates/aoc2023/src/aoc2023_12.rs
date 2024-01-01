use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use aoclib::{get_repo_root, Runner};

type Cache = HashMap<(Vec<char>, Vec<usize>), usize>;

#[derive(Default)]
pub struct Aoc2023_12 {
    input: PathBuf,
    springs: Vec<Spring>
}

impl Aoc2023_12 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_12 {
    fn name(&self) -> (usize, usize) {
        (2023, 12)
    }

    fn set_input(&mut self, input: &str) {
        self.input = get_repo_root().join(input)
    }

    fn parse(&mut self) {
        aoclib::read_lines(&self.input).iter().for_each(|line| self.springs.push(line.parse().unwrap()));
    }

    fn part1(&mut self) -> i64 {
        self.springs.iter().map(|spring| spring.combos()).sum()
    }

    fn part2(&mut self) -> i64 {
        self.springs.iter().map(|spring| spring.expand().combos()).sum()
    }
}

#[derive(Debug)]
struct Spring {
    pattern: Vec<char>,
    sizes: Vec<usize>
}
impl Spring {
    fn expand(&self) -> Self {
        let p: String = self.pattern.iter().collect();
        let p = format!("{p}?{p}?{p}?{p}?{p}");
        let s: Vec<usize> = self.sizes.iter().cloned().cycle().take(5 * self.sizes.len()).collect();

        Spring { pattern: p.chars().collect(), sizes: s}
    }
    fn combos(&self) -> i64 {
        let mut cache = HashMap::new();
        Self::do_score(&self.pattern, &self.sizes, &mut cache) as i64
    }

    fn do_score(pattern: &[char], sizes: &[usize], cache: &mut Cache) -> usize {
        if let Some(result) = cache.get(&(pattern.to_vec(), sizes.to_vec())) {
            return *result;
        }
        if sizes.is_empty() {
            return (!pattern.contains(&'#')) as usize
        }

        let min_remaining = sizes.iter().sum::<usize>() + sizes.len() - 1;

        if pattern.len() < min_remaining {
            return 0;
        }


        let result = match pattern[0] {
            '.' => Self::do_score(&pattern[1..], sizes, cache),
            '#' => Self::do_hash(pattern, sizes, cache),
            '?' => Self::do_score(&pattern[1..], sizes, cache) + Self::do_hash(pattern, sizes, cache),
            _ => panic!("Invalid char in input")
        };

        cache.insert((pattern.to_vec(), sizes.to_vec()), result);
        result
    }

    fn do_hash(pattern: &[char], sizes: &[usize], cache: &mut Cache) -> usize {
        if pattern.len() < sizes[0] || pattern[0..sizes[0]].contains(&'.') {
            return 0;
        }

        if pattern.len() == sizes[0] && sizes.len() == 1 {
            return (sizes.len() == 1) as usize;
        }

        if pattern[sizes[0]] == '#' {
            return 0;
        }

        Self::do_score(&pattern[sizes[0]+1..], &sizes[1..], cache)
    }
}


impl FromStr for Spring {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pattern, sizes) = s.split_once(' ').unwrap();
        let pattern = pattern.chars().collect();
        let sizes = sizes.split(',').map(|num| num.parse().unwrap()).collect();

        Ok(Spring {pattern, sizes})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s: Spring = "???.### 1,1,3".parse().unwrap();
        assert_eq!(1, s.combos());
    }

    #[test]
    fn test2() {
        let s: Spring = ".??..??...?##. 1,1,3".parse().unwrap();
        assert_eq!(4, s.combos());
    }

    #[test]
    fn test3() {
        let s: Spring = "?#?#?#?#?#?#?#? 1,3,1,6".parse().unwrap();
        assert_eq!(1, s.combos());
    }

    #[test]
    fn test4() {
        let s: Spring = "????.#...#... 4,1,1".parse().unwrap();
        assert_eq!(1, s.combos());
    }

    #[test]
    fn test5() {
        let s: Spring = "????.######..#####. 1,6,5".parse().unwrap();
        assert_eq!(4, s.combos());
    }

    #[test]
    fn test6() {
        let s: Spring = "?###???????? 3,2,1".parse().unwrap();
        assert_eq!(10, s.combos());
    }

    #[test]
    fn part1() {
        let mut day = Aoc2023_12::new();

        day.set_input("crates/aoc2023/test/2023-12.txt");
        day.parse();
        let result = day.part1();

        assert_eq!(21, result);
    }

    #[test]
    fn part2() {
        let mut day = Aoc2023_12::new();

        day.set_input("crates/aoc2023/test/2023-12.txt");
        day.parse();
        let result = day.part2();

        assert_eq!(525152, result);
    }
}