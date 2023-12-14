use std::fmt::Display;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use regex::Regex;

pub fn read_lines<T: AsRef<Path>>(pathname: T) -> Vec<String> {
    read(pathname, "\n")
}

pub fn read_groups<T: AsRef<Path>>(pathname: T) -> Vec<String> {
    let regex = Regex::new(r"\n\n|\r\n\r\n").unwrap();

    let contents = read_full(pathname);
    regex
        .split(&contents)
        .filter(|s| !s.is_empty())
        .map(|s|s.to_string())
        .collect()
}

pub fn read_full<T: AsRef<Path>>(pathname: T) -> String {
    read_to_string(pathname)
        .expect("unable to open file")
}

pub fn read<T: AsRef<Path>>(pathname: T, separator: &str) -> Vec<String> {
    read_to_string(pathname)
        .expect("unable to open file")
        .split(separator)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub fn lcm_of(list: Vec<u64>) -> u64 {
    list.iter().fold(1, |acc, x| lcm(acc, *x))
}

pub fn gcd(a: u64, b: u64) -> u64 {
    // Calculate the greatest common divisor using the Euclidean Algorithm
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    // Calculate the lowest common multiple using the Greatest Common Divisor
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a,b)
    }
}

pub fn get_repo_root() -> PathBuf {
    let path = std::env::current_exe()
        .expect("Failed to get executable path");
    path.parent().unwrap()
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap()
        .to_path_buf()
}

pub fn output<T: Display>(output: T) -> Vec<String> {
    vec![format!("{}", output)]
}

pub enum Selector {
    All,
    One(usize),
    Last,
}

pub trait Runner {
    fn name(&self) -> (usize, usize);
    fn set_input(&mut self, input: &str);
    fn parse(&mut self);
    fn part1(&mut self) -> u64;
    fn part2(&mut self) -> u64;
}

pub fn run_solution<T: Runner + ?Sized>(solution: &mut T) {
    let name = solution.name();
    println!("---- {}, Day {} ----", name.0, name.1);

    solution.set_input(&format!("aoc/crates/aoc{}/input/{}-{:02}.txt", name.0, name.0, name.1));

    let start = Instant::now();
    solution.parse();
    let parse_time = start.elapsed();
    println!("{} Parsing", get_duration_string(parse_time));

    let start = Instant::now();
    let p1 = solution.part1();
    let p1_str = vec![format!("{p1}")];
    let p1_time = start.elapsed();
    print_solution(1, &p1_str, p1_time);

    let start = Instant::now();
    let p2 = solution.part2();
    let p2_str = vec![format!("{p2}")];
    let p2_time = start.elapsed();
    print_solution(2, &p2_str, p2_time);
}

fn print_solution(which: usize, output: &[String], duration: Duration) {


    let mut i = output.iter();

    println!(
        "{} Part {which}: {}",
        get_duration_string(duration),
        i.next().unwrap()
    );

    for line in i {
        println!("{:16}{line}", "");
    }
}

fn get_duration_string(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let milliseconds = duration.subsec_millis();
    let microseconds = duration.subsec_micros() % 1000;

    format!("{seconds:3}.{milliseconds:03}.{microseconds:03}")
}