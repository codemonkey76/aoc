use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_03 {
    lines: Vec<String>
}

impl Aoc2023_03 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_03 {
    fn name(&self) -> (usize, usize) {
        (2023, 3)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines(aoclib::get_input_path(self.name()));
        self.lines = lines;
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output("Not yet implemented")
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("Not yet implemented")
    }
}