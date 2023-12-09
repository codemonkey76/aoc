use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_02 {
    games: Vec<Game>
}

impl Aoc2023_02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_02 {
    fn name(&self) -> (usize, usize) {
        (2023, 2)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines(aoclib::get_input_path(self.name()));
        for line in lines {
            let (game, turns) = line.split_once(": ").unwrap();
            let (_, game_id) = game.split_once(" ").unwrap();

            let turns = turns.split("; ").collect::<Vec<_>>();
            let mut turn_list = Vec::new();
            for t in turns {
                let cubes = t.split(", ").collect::<Vec<_>>();
                let mut turn = Turn::default();
                for cube in cubes {
                    let (amount, color) = cube.split_once(" ").unwrap();
                    let amount: usize = amount.parse().unwrap();

                    match &color[0..1] {
                        "r" => turn.red = amount,
                        "g" => turn.green = amount,
                        "b" => turn.blue = amount,
                        _ => panic!("Bug detected")
                    }
                }
                turn_list.push(turn);
            }
            self.games.push(Game::new(game_id.parse().unwrap(), turn_list));
        }
    }

    fn part1(&mut self) -> Vec<String> {

        let rule = Turn {
            red: 12,
            green: 13,
            blue: 14
        };

        let total: usize = self.games
            .iter()
            .filter(|game| game.turns.iter().all(|turn| turn.is_valid(&rule)))
            .map(|game| game.id)
            .sum();


        aoclib::output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        let power_sum: usize =
            self.games
                .iter()
                .map(|game| {
                    let (red, green, blue) = game.turns
                        .iter()
                        .fold((0, 0, 0), |acc, turn| {
                            (
                                acc.0.max(turn.red),
                                acc.1.max(turn.green),
                                acc.2.max(turn.blue)
                            )
                        });
                    red * green * blue
                })
                .sum();
        
        aoclib::output(power_sum)
    }
}

#[derive(Debug, Default)]
struct Game {
    id: usize,
    turns: Vec<Turn>
}

impl Game {
    pub fn new(id: usize, turns: Vec<Turn>) -> Self {
        Game {
            id,
            turns
        }
    }
}

#[derive(Debug, Default)]
struct Turn {
    red: usize,
    green: usize,
    blue: usize

}

impl Turn {
    pub fn is_valid(&self, rule: &Turn) -> bool {
        self.red <= rule.red && self.green <= rule.green && self.blue <= rule.blue
    }
}