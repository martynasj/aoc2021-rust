use std::{self, collections::HashMap, hash::Hash};

fn main() {
    let lines: Vec<Line> = deserialize_lines(&std::fs::read_to_string("./input.txt").unwrap())
        .into_iter()
        .filter(|x| x.is_perpendicular())
        .collect();

    let grid = Grid::new_from_lines(lines);

    println!("{}", grid.get_overlapping_point_count());
}

fn deserialize_lines(input: &str) -> Vec<Line> {
    let result = input
        .lines()
        .map(|l| {
            let (start, end) = l.split_once("->").unwrap();
            let start = start.trim().split_once(",").unwrap();
            let end = end.trim().split_once(",").unwrap();

            Line(Point::pos_from_str(start), Point::pos_from_str(end))
        })
        .collect();

    result
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Point(usize, usize);

impl Point {
    fn pos_from_str(start: (&str, &str)) -> Self {
        let x1: usize = start.0.parse().unwrap();
        let y1: usize = start.1.parse().unwrap();

        Point(x1, y1)
    }
}

struct Line(Point, Point);

impl Line {
    fn is_horizontal(&self) -> bool {
        self.0 .1 == self.1 .1
    }

    fn is_vertical(&self) -> bool {
        self.0 .0 == self.1 .0
    }

    fn is_perpendicular(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    fn get_all_points(&self) -> Vec<Point> {
        if self.is_horizontal() {
            let point_range = if self.0 .0 < self.1 .0 {
                self.0 .0..=self.1 .0
            } else {
                self.1 .0..=self.0 .0
            };
            return point_range.map(|x| return Point(x, self.0 .1)).collect();
        }
        let point_range = if self.0 .1 < self.1 .1 {
            self.0 .1..=self.1 .1
        } else {
            self.1 .1..=self.0 .1
        };
        return point_range.map(|x| return Point(self.0 .0, x)).collect();
    }
}

struct Grid {
    lines: Vec<Line>,
    tracker: HashMap<Point, u8>,
}

impl Grid {
    fn new_from_lines(lines: Vec<Line>) -> Self {
        let mut grid = Grid {
            lines: vec![],
            tracker: HashMap::new(),
        };

        lines.into_iter().for_each(|l| grid.place_line(l));

        grid
    }

    fn place_line(&mut self, line: Line) {
        // mark the grid
        let line_points = line.get_all_points();
        line_points.iter().for_each(|p| {
            let existing = self.tracker.entry(p.clone()).or_insert(0);
            *existing += 1;
        });

        self.lines.push(line);
    }

    fn get_overlapping_point_count(&self) -> usize {
        self.tracker.iter().filter(|(_, &val)| val >= 2).count()
    }

    fn print_grid(&self) {
        let mut grid = [[0; 10]; 10];

        self.tracker
            .iter()
            .for_each(|(pt, &count)| grid[pt.1][pt.0] = count);

        grid.iter().for_each(|l| {
            let str: String = l.iter().map(|&p| return p.to_string()).collect();
            println!("{}", str);
        })
    }
}

#[test]
fn test_line_get_all_points() {
    let l1 = Line(Point(1, 1), Point(3, 1));
    assert_eq!(l1.get_all_points().len(), 3);
    assert_eq!(l1.get_all_points().get(0).unwrap().0, 1);
    assert_eq!(l1.get_all_points().get(1).unwrap().0, 2);
    assert_eq!(l1.get_all_points().get(2).unwrap().0, 3);

    let l2 = Line(Point(1, 1), Point(1, 3));
    assert_eq!(l2.get_all_points().len(), 3);
    assert_eq!(l2.get_all_points().get(0).unwrap().1, 1);
    assert_eq!(l2.get_all_points().get(1).unwrap().1, 2);
    assert_eq!(l2.get_all_points().get(2).unwrap().1, 3);

    let l3 = Line(Point(3, 4), Point(1, 4));
    assert_eq!(l3.get_all_points().len(), 3);
    assert_eq!(l3.get_all_points().get(0).unwrap().1, 3);
    assert_eq!(l3.get_all_points().get(1).unwrap().1, 2);
    assert_eq!(l3.get_all_points().get(2).unwrap().1, 1);
}

#[test]
fn test_place_line_on_grid() {
    let l1 = Line(Point(0, 9), Point(5, 9));
    let l2 = Line(Point(0, 9), Point(2, 9));
    let mut grid = Grid::new_from_lines(vec![l1, l2]);
    assert_eq!(grid.get_overlapping_point_count(), 3);

    let l3 = Line(Point(5, 8), Point(5, 9));
    grid.place_line(l3);
    assert_eq!(grid.get_overlapping_point_count(), 4);
}
