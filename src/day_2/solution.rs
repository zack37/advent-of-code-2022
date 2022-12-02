use crate::common::read_file_to_string;
#[derive(Copy, Clone, Debug)]
enum Choice {
    Rock(usize),
    Paper(usize),
    Scissors(usize),
}

struct Game {
    rock: Choice,
    paper: Choice,
    scissors: Choice,
    points_for_loss: usize,
    points_for_draw: usize,
    points_for_win: usize,
}

impl Game {
    fn new() -> Game {
        Game {
            rock: Choice::Rock(1),
            paper: Choice::Paper(2),
            scissors: Choice::Scissors(3),
            points_for_loss: 0,
            points_for_draw: 3,
            points_for_win: 6,
        }
    }

    fn parse_choice(&self, choice: &str) -> Choice {
        match choice {
            "A" | "X" => self.rock,
            "B" | "Y" => self.paper,
            "C" | "Z" => self.scissors,
            _ => panic!("Invalid choice"),
        }
    }

    fn calculate_score(&self, pair: (Choice, Choice)) -> usize {
        match pair {
            (Choice::Rock(_), Choice::Rock(b)) => b + self.points_for_draw,
            (Choice::Rock(_), Choice::Paper(b)) => b + self.points_for_win,
            (Choice::Rock(_), Choice::Scissors(b)) => b + self.points_for_loss,
            (Choice::Paper(_), Choice::Rock(b)) => b + self.points_for_loss,
            (Choice::Paper(_), Choice::Paper(b)) => b + self.points_for_draw,
            (Choice::Paper(_), Choice::Scissors(b)) => b + self.points_for_win,
            (Choice::Scissors(_), Choice::Rock(b)) => b + self.points_for_win,
            (Choice::Scissors(_), Choice::Paper(b)) => b + self.points_for_loss,
            (Choice::Scissors(_), Choice::Scissors(b)) => b + self.points_for_draw,
        }
    }

    fn calculate_move(&self, choice: Choice, outcome: &str) -> Choice {
        match (choice, outcome) {
            (Choice::Rock(_), "X") => self.scissors,
            (Choice::Rock(_), "Y") => self.rock,
            (Choice::Rock(_), "Z") => self.paper,
            (Choice::Paper(_), "X") => self.rock,
            (Choice::Paper(_), "Y") => self.paper,
            (Choice::Paper(_), "Z") => self.scissors,
            (Choice::Scissors(_), "X") => self.paper,
            (Choice::Scissors(_), "Y") => self.scissors,
            (Choice::Scissors(_), "Z") => self.rock,
            _ => unreachable!("Invalid outcome"),
        }
    }

    pub fn day_1(self) -> usize {
        let input = read_file_to_string("./src/day_2/part_1_input.txt");
        input
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|x| {
                let parts = x.split(" ").collect::<Vec<&str>>();
                let pair = (self.parse_choice(parts[0]), self.parse_choice(parts[1]));
                self.calculate_score(pair)
            })
            .sum()
    }

    pub fn day_2(self) -> usize {
        let input = read_file_to_string("./src/day_2/part_2_input.txt");
        input
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|x| {
                let parts = x.split(" ").collect::<Vec<&str>>();
                let their_move = self.parse_choice(parts[0]);
                let pair = (
                    self.parse_choice(parts[0]),
                    self.calculate_move(their_move, parts[1]),
                );

                self.calculate_score(pair)
            })
            .sum()
    }
}

pub fn part_1() -> usize {
    let game = Game::new();
    game.day_1()
}

pub fn part_2() -> usize {
    let game = Game::new();
    game.day_2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_solution() {
        assert_eq!(part_1(), 15422);
    }

    #[test]
    fn part_2_solution() {
        assert_eq!(part_2(), 15442);
    }
}
