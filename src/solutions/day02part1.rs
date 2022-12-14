use std::fs;

pub fn solve() {
    let content = fs::read_to_string("puzzle_input/day02.txt").expect("Couldn't read file");

    let points: i32 = content
        .lines()
        .map(|round| convert_to_points(round))
        .sum();

    println!("{}", points);
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

#[derive(PartialEq)]
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
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
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

    fn points(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

pub fn convert_to_points(round: &str) -> i32 {
    let mut shapes = round.split(" ");

    let opponent = Shape::build(
        shapes.next().expect("Round with no opponent")
    );
    let player = Shape::build(
        shapes.next().expect("Round with no player")
    );

    let shape_points = player.points();

    return match player.get_outcome_vs(opponent) {
        Outcome::Win => shape_points + 6,
        Outcome::Draw => shape_points + 3,
        Outcome::Loss => shape_points + 0,
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_points_win() {
        let round = "A Y";
        assert_eq!(8, convert_to_points(round));
    }

    #[test]
    fn convert_to_points_loss() {
        let round = "B X";
        assert_eq!(1, convert_to_points(round));
    }

    #[test]
    fn convert_to_points_draw() {
        let round = "C Z";
        assert_eq!(6, convert_to_points(round));
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