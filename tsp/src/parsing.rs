use crate::points::{Point, Points};

pub fn parse_problem_from_str(text: &str) -> Result<Points, String> {
    let lines = text.lines();

    let lines = lines
        .skip_while(|line| !line.starts_with("NODE_COORD_SECTION"))
        .skip(1);

    let mut points = Vec::new();

    for line in lines {
        if line == "EOF" {
            break;
        }

        let Ok(&[_n, x, y]) = line
            .split_whitespace()
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .as_deref() else { return Err("Error while parsing points, line: ".to_string() + line);};

        points.push(Point::new(x, y))
    }

    Ok(Points::from_points(points))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let text = include_str!("../../vlsi/xqf131.tsp");

        let points = parse_problem_from_str(text).unwrap();

        dbg!(assert_eq!(points.list.len(), 131));
    }
}
