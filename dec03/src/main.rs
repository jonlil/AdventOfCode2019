use std::ops::{Add, Sub};
use std::collections::BTreeMap;
use std::io::{self, BufRead};

use draw::{
    Drawing,
    Canvas,
    Style,
    Shape,
    render,
    SvgRenderer,
    Color,
    shape::{
        LineBuilder,
        Line,
    },
};

//fn edge_coordinates(x1: i32, y1: i32, x2: i32, y2) {
//    if p1.x > max_x {
//        max_x = p1.x;
//    } else if p1.x < min_x {
//        min_x = p1.x;
//    }
//    if p2.x > max_x {
//        max_x = p2.x;
//    } else if p2.x < min_x {
//        min_x = p2.x;
//    }
//
//    if p1.y > max_y {
//        max_y = p1.y;
//    } else if p1.y < min_y {
//        min_y = p1.y;
//    }
//    if p2.y > max_y {
//        max_y = p2.y;
//    } else if p2.y < min_y {
//        min_y = p2.y;
//    }
//}

static X_OFFSET: isize = 11856;
static Y_OFFSET: isize = 7695;

fn offset_x(origin: isize) -> f32 {
    ((origin + X_OFFSET) / 10) as f32
}

fn offset_y(origin: isize) -> f32 {
    ((origin + Y_OFFSET) / 10) as f32
}

fn render_wires(wires: Vec<Wire>) {
    let mut canvas = Canvas::new(1400, 1100);

    for wire in wires {
        let color = Color::random();
        let shapes: Vec<Drawing> = wire.coordinates()
            .windows(2)
            .into_iter()
            .map(|points| {
                let p1 = points[0];
                let p2 = points[1];

                let shape = LineBuilder::new(
                        offset_x(p1.x),
                        offset_y(p1.y),
                    )
                    .line_to(offset_x(p2.x), offset_y(p2.y))
                    .build();

                Drawing::new()
                    .with_shape(shape)
                    .with_style(Style::stroked(5, color))
            }).collect();

        for shape in shapes {
            canvas.display_list.add(shape);
        }
    }

    canvas.display_list.add(
        Drawing::new()
            .with_shape(Shape::Circle { radius: 20 })
            .with_xy(offset_x(1151), offset_y(134))
            .with_style(Style::filled(Color::black()),
    ));
    canvas.display_list.add(
        Drawing::new()
            .with_shape(Shape::Circle { radius: 20 })
            .with_xy(offset_x(0), offset_y(0))
            .with_style(Style::filled(Color::black()),
    ));

    // save the canvas as an svg
    render::save(
        &canvas,
        "tests/svg/basic_end_to_end.svg",
        SvgRenderer::new(),
    )
    .expect("Failed to save");
}


fn main() {
    let stdin = io::stdin();

    let mut wires: Vec<Wire> = vec![];
    for line in stdin.lock().lines() {
        wires.push(Wire::from(line.unwrap().as_ref()));
    }

    let closest_intersection = find_closest_intersection(wires);
    eprintln!("{:#?}", closest_intersection);
    //render_wires(wires);
}

type DirectionDistance = (Direction, u32);

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    fn manhattan_distance(self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

impl From<(Point, DirectionDistance)> for Point {
    fn from(from: (Point, DirectionDistance)) -> Point {
        let source_point = from.0;
        let target = from.1;

        match target.0 {
            Direction::Right => source_point + Point::new(target.1 as isize, 0),
            Direction::Down => source_point - Point::new(0, target.1 as isize),
            Direction::Left => source_point - Point::new(target.1 as isize, 0),
            Direction::Up => source_point + Point::new(0, target.1 as isize),
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Wire {
    points: Vec<Point>,
}

impl Wire {
    fn coordinates(&self) -> Vec<Point> {
        self.points.windows(2)
            .map(|points| {
                let coordinates = coordinates_between_points(points);
                coordinates[1..].to_vec()
            })
            .flatten()
            .collect()
    }
}

impl From<Vec<DirectionDistance>> for Wire {
    fn from(data: Vec<DirectionDistance>) -> Wire {
        let mut points: Vec<Point> = Vec::with_capacity(data.len() + 1);
        points.push(Point { x: 0, y: 0 });

        for item in data {
            points.push(Point::from((
                points[points.len() - 1],
                item,
            )));
        }

        Wire {
            points: points,
        }
    }
}

impl From<&str> for Wire {
    fn from(input: &str) -> Wire {
        let paths = parse_path(input)
            .map(parse_direction_and_distance)
            .collect::<Vec<DirectionDistance>>();

        Wire::from(paths)
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn heading(x: f32, y: f32, tx: f32, ty: f32) -> i32 {
    (ty - y).atan2(tx - x).to_degrees() as i32
}

fn coordinates_between_points(points: &[Point]) -> Vec<Point> {
    let p1 = points[0];
    let p2 = points[1];

    match heading(p1.x as f32, p1.y as f32, p2.x as f32, p2.y as f32) {
        // right
        0 => (p1.x..=p2.x).map(|x| Point::new(x, p1.y)).collect(),
        // left
        180 => (p2.x..=p1.x).map(|x| Point::new(x, p1.y)).rev().collect(),
        // up
        90 => (p1.y..=p2.y).map(|y| Point::new(p1.x, y)).collect(),
        // down
        -90 => (p2.y..=p1.y).map(|y| Point::new(p1.x, y)).rev().collect(),
        _ => panic!("We only support sharp angles"),
    }
}

fn deduplicate_wire_coordinates<'a, I>(coordinates: I) -> Vec<Point>
where
    I: Iterator<Item = Point>,
{
    let unique_map: BTreeMap<Point, u32> = BTreeMap::new();
    coordinates
        .fold(unique_map, |mut acc, coordinate| {
            if !acc.contains_key(&coordinate) {
                acc.insert(coordinate, 1);
            }

            acc
        })
        .iter()
        .map(|(key, _value)| *key)
        .collect()
}

fn find_intersection_points(wires: Vec<Wire>) -> Vec<Point> {
    let map: BTreeMap<Point, u32> = BTreeMap::new();
    wires.into_iter()
        .map(|wire| {
            deduplicate_wire_coordinates(
                wire.coordinates()
                    .into_iter()
                    .skip(1)
                )
        })
        .flatten()
        .fold(map, |mut acc, coordinate| {
            if let Some(x) = acc.get_mut(&coordinate) {
                *x += 1;
            } else {
                acc.insert(coordinate, 1);
            }

            acc
        })
        .iter()
        .filter(|(_key, value)| *value > &1)
        .map(|(key, _value)| *key)
        .collect()
}

fn find_closest_intersection(wires: Vec<Wire>) -> u32 {
    let mut intersection_distances: Vec<u32> = find_intersection_points(wires)
        .into_iter()
        .map(|coordinate| coordinate.manhattan_distance())
        .collect();
    intersection_distances.sort();
    intersection_distances[0]
}

fn parse_direction_and_distance(input: &str) -> (Direction, u32) {
    let direction = match &input.chars().nth(0) {
        Some('R') => Direction::Right,
        Some('D') => Direction::Down,
        Some('L') => Direction::Left,
        Some('U') => Direction::Up,
        None | _ => panic!("Unknown direction found"),
    };

    let distance = &input.chars()
        .into_iter()
        .skip(1)
        .map(|s| s.to_string())
        .collect::<String>();

    (direction, distance.parse::<u32>().unwrap())
}

fn parse_path(input: &str) -> impl Iterator<Item = &str> {
    input.split(',')
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_direction_and_distance_from_instructions() {
        let paths = parse_path(&"R75,D30,L83,U83")
            .map(parse_direction_and_distance)
            .collect::<Vec<DirectionDistance>>();

        assert_eq!(paths, vec![
            (Direction::Right, 75),
            (Direction::Down, 30),
            (Direction::Left, 83),
            (Direction::Up, 83),
        ]);
    }

    #[test]
    fn it_can_substract_points() {
        let p1 = Point {
            x: 0,
            y: 0,
        };

        assert_eq!(p1 - Point::new(5, 0), Point::new(-5, 0));
    }

    #[test]
    fn it_can_add_points() {
        let p1 = Point {
            x: 0,
            y: 0,
        };

        assert_eq!(p1 + Point::new(5, 0), Point::new(5, 0));
    }

    #[test]
    fn it_can_build_wire_points() {
        assert_eq!(
            Point::from((
                Point { x: 0, y: 0 },
                (Direction::Right, 75),
            )),
            Point { x: 75, y: 0 },
        );
        assert_eq!(
            Point::from((
                Point { x: 0, y: 0 },
                (Direction::Down, 75),
            )),
            Point { x: 0, y: -75 },
        );
        assert_eq!(
            Point::from((
                Point { x: 0, y: 0 },
                (Direction::Left, 75),
            )),
            Point { x: -75, y: 0 },
        );
        assert_eq!(
            Point::from((
                Point { x: 0, y: 0 },
                (Direction::Up, 75),
            )),
            Point { x: 0, y: 75 },
        );
    }

    #[test]
    fn it_can_parse_connection_paths_to_wire() {
        let paths = parse_path(&"R75,D30,L83,U83")
            .map(parse_direction_and_distance)
            .collect::<Vec<DirectionDistance>>();

        assert_eq!(
            Wire::from(paths),
            Wire {
                points: vec![
                    Point::new(0, 0),
                    Point::new(75, 0),
                    Point::new(75, -30),
                    Point::new(-8, -30),
                    Point::new(-8, 53),
                ],
            }
        );
    }

    #[test]
    fn it_can_fill_points_between_points() {
        let point1 = Point::new(0, 0);
        let point2 = Point::new(1, 0);

        assert_eq!(
            coordinates_between_points(&[point1, point2]),
            vec![
                Point::new(0, 0),
                Point::new(1, 0),
            ],
        );

        assert_eq!(
            coordinates_between_points(&[Point::new(0, 0), Point::new(-1, 0)]),
            vec![
                Point::new(0, 0),
                Point::new(-1, 0),
            ],
        );

        assert_eq!(
            coordinates_between_points(&[Point::new(0, 0), Point::new(0, -1)]),
            vec![
                Point::new(0, 0),
                Point::new(0, -1),
            ],
        );
    }

    #[test]
    fn it_can_get_coordinate_for_wire() {
        let wire1 = Wire::from("R2,U2");
        assert_eq!(
            wire1.coordinates(),
            vec![
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(2, 1),
                Point::new(2, 2),
            ],
        );
    }

    #[test]
    fn it_can_deduplicate_wire_coordinates() {
        let wire1 = Wire::from("R4,U4,L3,D4,R4");
        let coordinates = wire1.coordinates();
        let initial_coordinate_length = coordinates.len();
        let deduplicated_coordinates = deduplicate_wire_coordinates(coordinates.into_iter());

        assert!(initial_coordinate_length != deduplicated_coordinates.len());
        assert_eq!(
            deduplicated_coordinates.len(),
            15,
        );
    }

    #[test]
    fn it_can_find_wire_intersection_points_coordinates() {
        let wire1 = Wire::from("R8,U5,L5,D3");
        let wire2 = Wire::from("U7,R6,D4,L4");
        let intersection_points = find_intersection_points(vec![wire1, wire2]);

        assert_eq!(
            intersection_points,
            vec![Point::new(3, 3), Point::new(6, 5)],
        );
        assert_eq!(
            find_closest_intersection(vec![
                Wire::from("R8,U5,L5,D3"),
                Wire::from("U7,R6,D4,L4"),
            ]),
            6,
        );
    }

    #[test]
    fn it_can_find_intersection_closed_to_center() {
        let wire1 = Wire::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire2 = Wire::from("U62,R66,U55,R34,D71,R55,D58,R83");

        assert_eq!(
            find_closest_intersection(vec![wire1, wire2]),
            159,
        );

        let wire1 = Wire::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire2 = Wire::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(
            find_closest_intersection(vec![wire1, wire2]),
            135,
        );
    }
}
