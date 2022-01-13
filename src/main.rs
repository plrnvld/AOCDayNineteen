
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use core::result::IntoIter;
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

fn main() {
    let mut scanners = read_scanners();

    println!("Scanners read: {}", scanners.len());

    let all_fixed_points: Vec<Point> = Vec::new();

    scanners[0].fix_current_points();

    let mut fixed_scanners = vec!(&scanners[0]);

    while scanners.iter().filter(|&s| !s.is_fixed()).count() > 0 {
        let fixed: Vec<&Scanner> = scanners.iter().filter(|&s| s.is_fixed()).collect();
        let unfixed = scanners.iter().filter(|&s| !s.is_fixed());
        
        let mut matched = false;
        
        for u in unfixed {
            let (matches, rot) = matches_with_any_fixed(&u, &fixed_scanners);
            if matches {
                println!("matching");
                matched = true;
            }
        }

        if !matched {
            panic!("No match!");
        }
    }
}

fn matches_with_any_fixed(u:&Scanner, fixed: &Vec<&Scanner>) -> (bool, usize) {            
    for f in fixed {
        for rot in 0..24 {
            if f.matches_with_scanner(u, rot).len() >= 12 {
                return (true, rot);
            }
        }            
    }    

    return (false, 99);
}



fn read_scanners() -> Vec<Scanner> {
    let mut scanners = Vec::new();
    let mut temp_points: Vec<Point> = Vec::new();
    
    let mut lines = lines_from_file("Input.txt");

    let mut handle_line = |line: &str|  {
        let scan_start = "--- scanner ";
        
        if line.starts_with(scan_start) {

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
            let coord_parts = line.split(",");
            let mut point_parts = coord_parts.map(|t| t.parse::<i32>().unwrap());
            let point = Point{ x: point_parts.next().unwrap(), y: point_parts.next().unwrap(), z: point_parts.next().unwrap() };

            temp_points.push(point);
        }
    };

    lines.reverse();

    for l in lines {
        handle_line(&l);
    }

    scanners.reverse();
    
    return scanners;
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point {
    fn rot_along_x(&self) -> Point {
        return Point { x: self.x, y: -self.z, z: self.y };
    }

    fn rot_along_y(&self) -> Point {
        return Point { x: -self.z, y: self.y, z: self.x };
    }

    fn rot_along_z(&self) -> Point {
        return Point { x: -self.y, y: self.x, z: self.z };
    }

    fn all_point_rotations(&self) -> [Point; 24] {
        let mut rotated_points: [Point; 24] = Default::default();

        let mut index = 0;
        for f1 in BASE_ROTATIONS {
            for f2 in SECONDARY_ROTATIONS {
                rotated_points[index] = f2(f1(&self));
                index += 1;
            }
        }

        return rotated_points;
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

    return rotated_points;
}

#[derive(Debug)]
struct Scanner {
    pub num: u8,
    pub points: Vec<Point>,
    pub fixed_points: Vec<Point>
}

impl Scanner {
    pub fn new(num: u8) -> Scanner { 
        return Scanner {
            num: num , 
            points: Vec::new(),
            fixed_points: Vec::new()
        } 
    }

    pub fn is_fixed(&self) -> bool {
        return self.fixed_points.len() > 0;
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(Point { x: point.x, y: point.y, z: point.z })
    }

    pub fn set_fixed_points(&mut self, points_to_fix: &Vec<Point>) {
        for p in points_to_fix {
            self.fixed_points.push(p.clone());
        }
    }

    pub fn fix_current_points(&mut self) {
        let points_to_fix = &self.points;
        for p in points_to_fix {
            self.fixed_points.push(p.clone());
        }
    }

    pub fn matches_with(&mut self, points: Vec<Point>)  -> bool {
        return &self.fixed_points.intersect(points).len() >= &12;
    }

    pub fn matches_with_scanner(&self, scanner: &Scanner, index: usize) -> Vec<Point> {
        let points = get_rotated_points(index, &scanner.points);

        let union:Vec<Point> = self.fixed_points.intersect(points);

        println!("Overlapping with {}", union.len());

        return union;
    }
}
