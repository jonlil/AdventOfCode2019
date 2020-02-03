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

//fn render_wires(wires: Vec<Wire>) {
//    let mut canvas = Canvas::new(1400, 1100);
//
//    for wire in wires {
//        let color = Color::random();
//        let shapes: Vec<Drawing> = wire.coordinates()
//            .windows(2)
//            .into_iter()
//            .map(|points| {
//                let p1 = points[0];
//                let p2 = points[1];
//
//                let shape = LineBuilder::new(
//                        offset_x(p1.x),
//                        offset_y(p1.y),
//                    )
//                    .line_to(offset_x(p2.x), offset_y(p2.y))
//                    .build();
//
//                Drawing::new()
//                    .with_shape(shape)
//                    .with_style(Style::stroked(5, color))
//            }).collect();
//
//        for shape in shapes {
//            canvas.display_list.add(shape);
//        }
//    }
//
//    canvas.display_list.add(
//        Drawing::new()
//            .with_shape(Shape::Circle { radius: 20 })
//            .with_xy(offset_x(1151), offset_y(134))
//            .with_style(Style::filled(Color::black()),
//    ));
//    canvas.display_list.add(
//        Drawing::new()
//            .with_shape(Shape::Circle { radius: 20 })
//            .with_xy(offset_x(0), offset_y(0))
//            .with_style(Style::filled(Color::black()),
//    ));
//
//    // save the canvas as an svg
//    render::save(
//        &canvas,
//        "tests/svg/basic_end_to_end.svg",
//        SvgRenderer::new(),
//    )
//    .expect("Failed to save");
//}


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

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq, PartialOrd, Ord)]
struct DistancePoint {
    distance: u32,
    point: Point,
}

fn coordinates_with_unique_distance(coordinates: Vec<Point>) -> Vec<DistancePoint> {
    let mut unique_map: BTreeMap<Point, u32> = BTreeMap::new();

    coordinates.iter().enumerate()
        .map(|(index, point)| {
            let distance = match unique_map.contains_key(point) {
                true => *unique_map.get(point).unwrap(),
                false => {
                    unique_map.insert(*point, (index + 1) as u32);
                    (index + 1) as u32
                },
            };

            DistancePoint {
                distance,
                point: *point,
            }
        })
        .collect()
}

impl Wire {
    fn coordinates(&self) -> Vec<DistancePoint> {
        let mut unique_map: BTreeMap<Point, u32> = BTreeMap::new();
        self.points.windows(2)
            .map(|points| {
                let coordinates = coordinates_between_points(points);
                coordinates[1..].to_vec()
            })
            .flatten()

            .enumerate()
            .map(|(index, point)| {
                let distance = match unique_map.contains_key(&point) {
                    true => *unique_map.get(&point).unwrap(),
                    false => {
                        unique_map.insert(point, (index + 1) as u32);
                        (index + 1) as u32
                    },
                };

                DistancePoint {
                    distance,
                    point,
                }
            })
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

fn deduplicate_wire_coordinates<'a, I>(coordinates: I) -> Vec<DistancePoint>
where
    I: Iterator<Item = DistancePoint>,
{
    let unique_map: BTreeMap<DistancePoint, u32> = BTreeMap::new();
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

#[derive(Clone, Copy, Debug)]
struct IntersectionDistance {
    distance: (u32, u32),
    counter: u32,
}

fn find_intersection_points(wires: Vec<Wire>) -> Vec<u32> {
    let map: BTreeMap<Point, Vec<u32>> = BTreeMap::new();

    wires.into_iter()
        .map(|wire| {
            deduplicate_wire_coordinates(
                wire.coordinates()
                    .into_iter()
                    .skip(1)
                )
        })
        .flatten()
        .fold(map, |mut acc, distance_point| {
            if let Some(x) = acc.get_mut(&distance_point.point) {
                x.push(distance_point.distance);
            } else {
                acc.insert(distance_point.point, vec![distance_point.distance]);
            }

            acc
        })
        .iter()
        .filter(|(_key, value)| value.len() == 2)
        .map(|(_key, value)| (value[0] + value[1]) as u32)
        .collect()
}

fn find_closest_intersection(wires: Vec<Wire>) -> u32 {
    let mut intersection_distances: Vec<u32> = find_intersection_points(wires)
        .into_iter()
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

