use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use array_tool::vec::*;

static BASE_ROTATIONS: [fn(&Point) -> Point; 4] = [
    |p| p.clone(),
    |p| p.rot_along_x(),
    |p| p.rot_along_x().rot_along_x(),
    |p| p.rot_along_x().rot_along_x().rot_along_x()
];

static SECONDARY_ROTATIONS: [fn(Point) -> Point; 6] = [
    |p| p.clone(),
    |p| p.rot_along_z(),
    |p| p.rot_along_z().rot_along_z(),
    |p| p.rot_along_z().rot_along_z().rot_along_z(),
    |p| p.rot_along_y(),
    |p| p.rot_along_y().rot_along_y().rot_along_y(),
];

static EMPTY_POINT: Point = Point { x: 0, y: 0, z: 0 };

fn main() {
    let mut all_beacons: Vec<Point> = Vec::new();
    let mut scanners = read_scanners();

    println!("Scanners read: {}", scanners.len());
    
    let mut fixed_scanners: Vec<Scanner> = vec!(scanners.pop().unwrap());

    for p in &fixed_scanners[0].points {
        all_beacons.push(p.clone());
    }

    let mut unfixed_scanners: Vec<Scanner> = Vec::from(scanners).to_vec();
    
    while &unfixed_scanners.len() > &0 {
        for mut f in fixed_scanners.to_vec() {
            unfixed_scanners.retain(|u| {
                let (matched, scanner_center, beacon_points) = scanners_overlap(&mut f, u);
                if matched {
                    let scanner_to_fix = Scanner { 
                        num: u.num, 
                        points: beacon_points.to_vec(),
                        location: Some(scanner_center),
                        was_compared_with: Vec::new()
                    };

                    fixed_scanners.push(scanner_to_fix);
                    
                    for beacon in &beacon_points {
                        if !all_beacons.contains(&beacon) {
                            all_beacons.push(beacon.clone());
                        }
                    }
                }
                
                return !matched;
            });
        }

        fixed_scanners.remove(0);
    }

    println!("Finished: all scanners fixed! {} beacons!", all_beacons.len());
}

fn scanners_overlap(s1fixed:&mut Scanner, s2unfixed: &Scanner) -> (bool, Point, Vec<Point>) {
    if !s1fixed.was_compared_with.contains(&s2unfixed.num) {
        println!("Comparing {} with {}", s1fixed.num, s2unfixed.num);
        s1fixed.was_compared_with.push(s2unfixed.num);
            
        //for rot1 in 0..24 {
            let rotated_fixed = &s1fixed.points;
            //let rotated_points1 = get_rotated_points(rot1, &s1.points);

            for rot2 in 0..24 {
                let rotated_unfixed = get_rotated_points(rot2, &s2unfixed.points);

                let (overlapping, scanner_center, translated_unfixed_points) = points_overlap(&rotated_fixed, &rotated_unfixed);
                if overlapping {
                    println!("  Matching!");
                    return (true, scanner_center, translated_unfixed_points);
                }
            }
        //}
    }

    return (false, EMPTY_POINT, Vec::new());
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point {
    fn rot_along_x(&self) -> Point {
        Point { x: self.x, y: -self.z, z: self.y }
    }

    fn rot_along_y(&self) -> Point {
        Point { x: -self.z, y: self.y, z: self.x }
    }

    fn rot_along_z(&self) -> Point {
        Point { x: -self.y, y: self.x, z: self.z }
    }

    fn add(&self, other: &Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }

    fn sub(&self, other: &Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }

    fn neg(&self) -> Point {
        Point { x: -self.x, y: -self.y, z: -self.z }
    }
}

fn get_rotated_points(index: usize, points: &Vec<Point>) -> Vec<Point> {
    let base_index: usize = index / 6;
    let secondary_index: usize = index % 6;

    let f_1 = BASE_ROTATIONS[base_index];
    let f_2 = SECONDARY_ROTATIONS[secondary_index];

    let mut rotated_points: Vec<Point> = Vec::new();

    for p in points {
        rotated_points.push(f_2(f_1(p)))
    }

    rotated_points
}

fn points_overlap(points1: &Vec<Point>, points2: &Vec<Point>) -> (bool, Point, Vec<Point>) {
    for p1 in points1 {
        for p2 in points2 {
            let translation = &p1.sub(&p2);

            let mut translated_p2s: Vec<Point> = Vec::new();
            for to_translate in points2 {
                translated_p2s.push(to_translate.add(translation));
            }

            let intersection: &Vec<Point> = &points1.intersect(translated_p2s.to_vec());

            let overlapping = intersection.len();

            if overlapping >= 12 {
                let scanner_center = p1.add(&p2.neg());
                return (true, scanner_center, translated_p2s);
            }
        }
    }    
    
    return (false, EMPTY_POINT, Vec::new());
}

#[derive(Debug, Clone)]
struct Scanner {
    pub num: u8,
    pub points: Vec<Point>,
    pub location: Option<Point>,
    pub was_compared_with: Vec<u8>
}

impl Scanner {
    pub fn new(num: u8) -> Scanner { 
        Scanner {
            num, 
            points: Vec::new(),
            location: None,
            was_compared_with: Vec::new()
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(Point { x: point.x, y: point.y, z: point.z })
    }
}

fn read_scanners() -> Vec<Scanner> {
    let mut scanners = Vec::new();
    let mut temp_points: Vec<Point> = Vec::new();    
    let mut lines = lines_from_file("Input.txt");

    let mut handle_line = |line: &str| {
        if line.starts_with("--- scanner ") {
            let scanner_nums: Vec<&str> = line.split(" ").collect();
            let scanner_num: u8 = scanner_nums.get(2).unwrap().parse::<u8>().unwrap();
            let mut scanner: Scanner = Scanner::new(scanner_num);
            let collected_points = temp_points.drain(..);

            if collected_points.len() > 0 {
                for p in collected_points {
                    scanner.add_point(p)
                }

                scanners.push(scanner);
            }                       
        } else if line.len() > 0 {
            let mut point_parts = line.split(",").map(|t| t.parse::<i32>().unwrap());
            
            temp_points.push(Point { 
                x: point_parts.next().unwrap(), 
                y: point_parts.next().unwrap(), 
                z: point_parts.next().unwrap() 
            });
        }
    };

    lines.reverse();

    for l in lines { 
        handle_line(&l); 
    }

    scanners.reverse();
    
    scanners
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename)
        .expect("no such file");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}