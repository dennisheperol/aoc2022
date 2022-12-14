use std::fs;

pub fn solve() {
    let content = fs::read_to_string("puzzle_input/day02.txt").expect("Couldn't read file");

    let points: i32 = content
        .lines()
        .map(|round| convert_to_points(round))
        .sum();

    println!("{}", points);
}

#[derive(PartialEq, Debug)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn build(s: &str) -> Outcome {
        match s {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Can't convert {s} to outcome.")
        }
    }
}

#[derive(PartialEq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn build(s: &str) -> Shape {
        match s {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Can't convert {s} to shape.")
        }
    }

    fn get_wins_vs(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Scissors => Shape::Paper,
            Shape::Paper => Shape::Rock,
        }
    }

    fn get_loses_vs(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Scissors => Shape::Rock,
            Shape::Paper => Shape::Scissors,
        }
    }

    fn get_outcome_vs(&self, other : Shape) -> Outcome {
        return if self.get_wins_vs() == other {
            Outcome::Win
        } else if self.get_loses_vs() == other {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }

    fn get_shape_for_outcome(&self, outcome: Outcome) -> Shape {
        match outcome {
            Outcome::Draw => self.clone(),
            Outcome::Win => self.get_loses_vs(),
            Outcome::Loss => self.get_wins_vs(),
        }
    }

    fn points(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

pub fn convert_to_points(round: &str) -> i32 {
    let mut instructions = round.split(" ");

    let opponent = Shape::build(
        instructions.next().expect("Round with no opponent")
    );
    let outcome = Outcome::build(
        instructions.next().expect("Round with no outcome")
    );

    let player = opponent.get_shape_for_outcome(outcome);

    return match player.get_outcome_vs(opponent) {
        Outcome::Win => player.points() + 6,
        Outcome::Draw => player.points() + 3,
        Outcome::Loss => player.points() + 0,
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_points_win() {
        let round = "A Y";
        assert_eq!(4, convert_to_points(round));
    }

    #[test]
    fn convert_to_points_loss() {
        let round = "B X";
        assert_eq!(1, convert_to_points(round));
    }

    #[test]
    fn convert_to_points_draw() {
        let round = "C Z";
        assert_eq!(7, convert_to_points(round));
    }

    #[test]
    fn get_outcome_vs_win() {
        let shape = Shape::Rock;
        assert_eq!(Outcome::Win, shape.get_outcome_vs(Shape::Scissors));
    }

    #[test]
    fn get_outcome_vs_loss() {
        let shape = Shape::Rock;
        assert_eq!(Outcome::Loss, shape.get_outcome_vs(Shape::Paper));
    }

    #[test]
    fn get_outcome_vs_draw() {
        let shape = Shape::Rock;
        assert_eq!(Outcome::Draw, shape.get_outcome_vs(Shape::Rock));
    }
}