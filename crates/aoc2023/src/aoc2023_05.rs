use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_05 {
    lines: Vec<String>
}

impl Aoc2023_05 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_05 {
    fn name(&self) -> (usize, usize) {
        (2023, 5)
    }

    fn parse(&mut self) {
        self.lines = aoclib::read_lines(aoclib::get_input_path(self.name()));
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output("Not Implemented")
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("Not Implemented")
    }
}