use std::fmt::Display;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub fn read_lines<T: AsRef<Path>>(pathname: T) -> Vec<String> {
    read_to_string(pathname)
        .expect("unable to open file")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub fn read_groups<T: AsRef<Path>>(pathname: T) -> Vec<String> {
    read_to_string(pathname)
        .expect("unable to open file")
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub fn get_input_path(info: (usize, usize)) -> PathBuf {
    let path = format!("crates/aoc{}/input/{}-{:02}.txt", info.0, info.0, info.1);
    println!("{}", path);
    std::env::current_exe()
        .expect("Failed to get executable path")
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap()
        .join(path)
}

pub fn get_input_path(info: (usize, usize)) -> PathBuf {
    get_path(info, "input")
}

pub fn get_test_path(info: (usize, usize)) -> PathBuf {
    get_path(info, "test")
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
    fn parse(&mut self);
    fn part1(&mut self) -> Vec<String>;
    fn part2(&mut self) -> Vec<String>;
}

pub fn run_solution<T: Runner + ?Sized>(solution: &mut T) {
    let name = solution.name();
    println!("---- {}, Day {} ----", name.0, name.1);

    let start = Instant::now();
    solution.parse();
    let parse_time = start.elapsed().as_millis();
    println!("{:3}.{:03} Parsing", parse_time / 1000, parse_time % 1000);

    let start = Instant::now();
    let p1 = solution.part1();
    let p1_time = start.elapsed();
    print_solution(1, &p1, p1_time);

    let start = Instant::now();
    let p2 = solution.part2();
    let p2_time = start.elapsed();
    print_solution(2, &p2, p2_time);
}

fn print_solution(which: usize, output: &[String], duration: Duration) {
    let seconds = duration.as_secs();
    let milliseconds = duration.subsec_millis();
    let microseconds = duration.subsec_micros() % 1000;

    let mut i = output.iter();

    println!(
        "{seconds:3}.{milliseconds:03}.{microseconds:03} Part {which}: {}",
        i.next().unwrap()
    );

    for line in i {
        println!("{:16}{line}", "");
    }
}