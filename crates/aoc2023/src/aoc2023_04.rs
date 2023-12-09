use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_04 {
    lines: Vec<String>
}

impl Aoc2023_04 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_04 {
    fn name(&self) -> (usize, usize) {
        (2023, 4)
    }

    fn parse(&mut self) {
        self.lines = aoclib::read_lines(aoclib::get_input_path(self.name()));
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output("Unimplemented")
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("Unimplemented")
    }
}
