use std::{cmp::Ordering, str::FromStr};

use crate::lib::{advent::Result, utils::parse_lines};
use crate::Day;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl FromStr for Direction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use Direction::*;
        if let [d] = s.as_bytes() {
            match d {
                b'N' => Ok(North),
                b'S' => Ok(South),
                b'E' => Ok(East),
                b'W' => Ok(West),
                _ => Err("Failed to parse Direction".into()),
            }
        } else {
            Err("Empty string".into())
        }
    }
}

impl Direction {
    fn rotate(&self, deg: isize) -> Direction {
        if deg == 0 {
            return *self;
        }
        use Direction::*;
        if deg < 0 {
            let new_deg = deg + 90;
            match self {
                East => North.rotate(new_deg),
                South => East.rotate(new_deg),
                West => South.rotate(new_deg),
                North => West.rotate(new_deg),
            }
        } else {
            let new_deg = deg - 90;
            match self {
                East => South.rotate(new_deg),
                South => West.rotate(new_deg),
                West => North.rotate(new_deg),
                North => East.rotate(new_deg),
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    Dir(Direction, isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let [d, v @ ..] = s.as_bytes() {
            let v = std::str::from_utf8(v)?.parse()?;
            use Instruction::*;
            match d {
                b'L' => Ok(Left(v)),
                b'R' => Ok(Right(v)),
                b'F' => Ok(Forward(v)),
                d => Ok(Dir(std::str::from_utf8(&[*d])?.parse()?, v)),
            }
        } else {
            Err("Failed to parse".into())
        }
    }
}

#[derive(Clone)]
struct Ship {
    instructions: Vec<Instruction>,
    current_direction: Direction,
    current_position: (isize, isize),
    waypoint: (isize, isize),
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
            current_direction: Direction::East,
            current_position: (0, 0),
            waypoint: (10, -1),
        }
    }
}

impl Ship {
    fn forward_by(pos: (isize, isize), d: Direction, v: isize) -> (isize, isize) {
        let mut pos = pos;
        use Direction::*;
        match d {
            East => pos.0 += v,
            South => pos.1 += v,
            West => pos.0 -= v,
            North => pos.1 -= v,
        }
        pos
    }

    fn run(&mut self) {
        for ins in &self.instructions {
            use Instruction::*;
            match ins {
                Dir(d, v) => {
                    self.current_position = Self::forward_by(self.current_position, *d, *v)
                }
                Left(v) => self.current_direction = self.current_direction.rotate(-*v),
                Right(v) => self.current_direction = self.current_direction.rotate(*v),
                Forward(v) => {
                    self.current_position =
                        Self::forward_by(self.current_position, self.current_direction, *v)
                }
            }
        }
    }

    fn distance_from_start(&self) -> isize {
        self.current_position.0.abs() + self.current_position.1.abs()
    }

    fn rotate_point(point: (isize, isize), deg: isize) -> (isize, isize) {
        match deg.cmp(&0) {
            Ordering::Equal => point,
            Ordering::Less => Self::rotate_point((point.1, -point.0), deg + 90),
            Ordering::Greater => Self::rotate_point((-point.1, point.0), deg - 90),
        }
    }

    fn run_waypoint(&mut self) {
        for ins in &self.instructions {
            use Instruction::*;
            match ins {
                Dir(d, v) => {
                    use Direction::*;
                    match d {
                        East => self.waypoint.0 += v,
                        South => self.waypoint.1 += v,
                        West => self.waypoint.0 -= v,
                        North => self.waypoint.1 -= v,
                    }
                }
                Left(v) => self.waypoint = Self::rotate_point(self.waypoint, -*v),
                Right(v) => self.waypoint = Self::rotate_point(self.waypoint, *v),
                Forward(v) => {
                    self.current_position.0 += v * self.waypoint.0;
                    self.current_position.1 += v * self.waypoint.1;
                }
            }
        }
    }
}

#[derive(Default)]
pub struct Day12 {
    ship: Ship,
}

impl<'a> Day<'a> for Day12 {
    fn setup(&mut self, input: &'a str) -> Result<()> {
        self.ship.instructions = parse_lines(input)?;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let mut ship = self.ship.clone();
        ship.run();
        Ok(ship.distance_from_start().to_string())
    }

    fn part2(&self) -> Result<String> {
        let mut ship = self.ship.clone();
        ship.run_waypoint();
        Ok(ship.distance_from_start().to_string())
    }
}
