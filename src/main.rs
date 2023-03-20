use std::str::FromStr;
use anyhow::{Result, anyhow, Context};
fn get_input() -> &'static str {
    return "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 4,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,3";
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let result = s.split_once(",");
        if result.is_none() {
            return Err(anyhow!("Invalid input"));
        }

        let (x, y) = result.unwrap();
        let x: i32 = str::parse(x).context("Invalid x")?;
        let y: i32 = str::parse(y).context("Invalid y")?;

        return Ok(Point { x, y });
    }

}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let result = s.split_once(" -> ");
        if result.is_none() {
            return Err(anyhow!("Invalid input"));
        }

        let (p1, p2) = result.unwrap();
        let p1: Point = str::parse(p1).context("Invalid p1")?;
        let p2: Point = str::parse(p2).context("Invalid p2")?;

        return Ok(Line { p1, p2 });
    }
}

impl Line {
    fn is_horizontal_or_vertical(&self) -> bool {
        return self.p1.x == self.p2.x || self.p1.y == self.p2.y;
    }
}


fn main() {
    let lines = get_input()
        .lines()
        .flat_map(|x| str::parse(x))
        .filter(|x: &Line| x.is_horizontal_or_vertical())
        .collect::<Vec<Line>>();

    println!("{:?}", lines);
}
