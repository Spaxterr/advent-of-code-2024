use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq)]
enum PointType {
    EMPTY,
    OBSTACLE,
    GUARD,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Guard {
    facing: Direction,
    position: [u16; 2],
    map: Vec<Vec<MapPoint>>,
}

impl Guard {
    fn set_position(&mut self, x: u16, y: u16) {
        self.position = [x, y];
        self.map[y as usize][x as usize].visited = true;
    }

    fn get_next_point(&mut self) -> Option<&mut MapPoint> {
        let x = self.position[0] as usize;
        let y = self.position[1] as usize;

        let width = self.map[0].len() - 1;
        let height = self.map.len() - 1;

        let point;

        match self.facing {
            Direction::UP => {
                if y <= 0 {
                    point = Option::None;
                } else {
                    point = Option::from(&mut self.map[y - 1][x]);
                }
            }
            Direction::RIGHT => {
                if x == width {
                    point = Option::None;
                } else {
                    point = Option::from(&mut self.map[y][x + 1]);
                }
            }
            Direction::DOWN => {
                if y == height {
                    point = Option::None;
                } else {
                    point = Option::from(&mut self.map[y + 1][x]);
                }
            }
            Direction::LEFT => {
                if x <= 0 {
                    point = Option::None;
                } else {
                    point = Option::from(&mut self.map[y][x - 1]);
                }
            }
        }

        return point;
    }

    fn walk(&mut self) {
        let mut finished = false;

        while !finished {
            let mut x = self.position[0];
            let mut y = self.position[1];

            let next_point = self.get_next_point();
            if next_point.is_none() {
                finished = true;
            } else {
                let point = next_point.unwrap();
                if point.point_type == PointType::OBSTACLE {
                    match self.facing {
                        Direction::UP => {
                            self.facing = Direction::RIGHT;
                        }
                        Direction::RIGHT => {
                            self.facing = Direction::DOWN;
                        }
                        Direction::DOWN => {
                            self.facing = Direction::LEFT;
                        }
                        Direction::LEFT => {
                            self.facing = Direction::UP;
                        }
                    }
                } else {
                    match self.facing {
                        Direction::UP => {
                            y -= 1;
                        }
                        Direction::RIGHT => {
                            x += 1;
                        }
                        Direction::DOWN => {
                            y += 1;
                        }
                        Direction::LEFT => {
                            x -= 1;
                        }
                    }

                    self.set_position(x, y);
                }
            }
        }
    }
}

#[derive(Debug)]
struct MapPoint {
    point_type: PointType,
    visited: bool,
}

fn main() {
    let map = read_map("./input.txt");
    let (x, y) = find_guard_position(&map);

    let mut guard = Guard {
        facing: Direction::UP,
        position: [x, y],
        map,
    };

    guard.walk();

    let mut count = 0;
    for row in guard.map {
        for point in row {
            if point.visited {
                count += 1;
            }
        }
    }

    println!("{count}");
}

fn read_map(file_path: &str) -> Vec<Vec<MapPoint>> {
    let mut result: Vec<Vec<MapPoint>> = Vec::new();
    let contents = read_to_string(file_path).expect("Could not read file");

    for row in contents.split("\n").filter(|r| !r.is_empty()) {
        let mut points = Vec::<MapPoint>::new();
        for c in row.chars() {
            let point;
            match c {
                '.' => {
                    point = MapPoint {
                        point_type: PointType::EMPTY,
                        visited: false,
                    }
                }
                '^' => {
                    point = MapPoint {
                        point_type: PointType::GUARD,
                        visited: true,
                    }
                }
                _ => {
                    point = MapPoint {
                        point_type: PointType::OBSTACLE,
                        visited: false,
                    }
                }
            }
            points.push(point);
        }
        result.push(points);
    }

    return result;
}

fn find_guard_position(map: &Vec<Vec<MapPoint>>) -> (u16, u16) {
    let mut x = 0;
    let mut y = 0;
    for row in 0..map.len() {
        for point in 0..map[row].len() {
            if map[row][point].point_type == PointType::GUARD {
                y = row;
                x = point;
            }
        }
    }

    return (x as u16, y as u16);
}
