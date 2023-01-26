use nom::{
    bytes::streaming::tag,
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

struct Line {
    sensor: Point,
    beacon: Point,
}

impl Line {
    fn parse(i: &str) -> IResult<&str, Line> {
        // Maps a fn on the result of a parser
        map(
            // The parser
            separated_pair(
                preceded(tag("Sensor at "), Point::parse),
                tag(": closest beacon is at "),
                Point::parse,
            ),
            |(sensor, beacon)| Line { sensor, beacon },
        )(i)
    }
}

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn parse(i: &str) -> IResult<&str, Point> {
        map(
            separated_pair(
                preceded(tag("x="), nom::character::complete::i64),
                tag(", "),
                preceded(tag("y="), nom::character::complete::i64),
            ),
            |(x, y)| Point { x, y },
        )(i)
    }
}

// Sensor at x=1259754, y=1927417: closest beacon is at x=1174860, y=2000000
fn main() {
    for line in include_str!("input.txt").lines() {
        println!("{}", line);
    }
}
