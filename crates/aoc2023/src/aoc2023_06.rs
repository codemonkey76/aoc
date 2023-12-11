use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_06 {
    lines: Vec<String>
}

impl Aoc2023_06 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_06 {
    fn name(&self) -> (usize, usize) {
        (2023, 6)
    }

    fn parse(&mut self)
    {
        self.lines = aoclib::read_lines(aoclib::get_test_path(self.name()));
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("Unsolved")
    }
}