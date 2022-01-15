
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

fn main() {
    let mut scanners = read_scanners();

    println!("Scanners read: {}", scanners.len());

    let mut all_fixed_points: Vec<Point> = Vec::new();

    let mut fixed_scanners: Vec<Scanner> = vec!(scanners.pop().unwrap());
    fixed_scanners[0].fix_current_points();

    let mut unfixed_scanners: Vec<Scanner> = Vec::from(scanners).to_vec();
    
    while &unfixed_scanners.len() > &0 {
        let mut matched = false;
        let mut current_matching_points: Vec<Point> = Vec::new();
        // let mut current_matching_scanner = 99;

        for f in fixed_scanners.to_vec() {
            unfixed_scanners.retain(|s| {
                let vec_nums: Vec<String> = fixed_scanners.iter().map(|f|f.num.to_string()).collect();
                let vec_text = vec_nums.join(",");
                let intersecting_points = matches_with_fixed(&s, &f, &vec_text);
                matched = intersecting_points.len() > 0;
                if matched {
                    println!("  matching");
                    
                    for ip in intersecting_points {
                        if !all_fixed_points.contains(&ip) {
                            all_fixed_points.push(ip);
                        }

                        current_matching_points.push(ip);                   
                    }

                    let scanner_to_fix = Scanner { 
                        num: s.num, 
                        points: current_matching_points.to_vec(), 
                        fixed_points: current_matching_points.to_vec() 
                    };
                    fixed_scanners.push(scanner_to_fix);
                }

                return !matched;
            });
        }

        fixed_scanners.remove(0);
    }

    println!("Finished: all fixed points: {}", all_fixed_points.len());
}

fn matches_with_fixed(u:&Scanner, f: &Scanner, vec_text: &str) -> Vec<Point> {
    println!("Comparing {} with {}: all {}", u.num, f.num, vec_text);
        
    for rot in 0..24 {
        let intersecting_points: Vec<Point> = f.matches_with_scanner(u, rot);
        if intersecting_points.len() >= 12 {
            return intersecting_points;
        }
    }             

    return Vec::new();
}

fn read_scanners() -> Vec<Scanner> {
    let mut scanners = Vec::new();
    let mut temp_points: Vec<Point> = Vec::new();
    
    let mut lines = lines_from_file("Example.txt");

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

    fn add(&self, other: &Point) -> Point {
        return Point { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z };
    }

    fn sub(&self, other: &Point) -> Point {
        return Point { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z };
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

#[derive(Debug, Clone)]
struct Scanner {
    pub num: u8,
    pub points: Vec<Point>,
    pub fixed_points: Vec<Point>,
}

impl Scanner {
    pub fn new(num: u8) -> Scanner { 
        return Scanner {
            num, 
            points: Vec::new(),
            fixed_points: Vec::new()
        } 
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(Point { x: point.x, y: point.y, z: point.z })
    }

    pub fn fix_current_points(&mut self) {
        let points_to_fix = &self.points;
        for p in points_to_fix {
            self.fixed_points.push(p.clone());
        }
    }

    pub fn set_fixed_points(&mut self, points_to_fix: &Vec<Point>) {
        for p in points_to_fix {
            self.fixed_points.push(p.clone());
        }
    }

    pub fn matches_with_scanner(&self, other_scanner: &Scanner, index: usize) -> Vec<Point> {
        let other_points = get_rotated_points(index, &other_scanner.points);

        // println!("Comparing scanner {} with scanner {}", &self.num, &other_scanner.num);
        for self_point in &self.points {
            for other in &other_points {
                let translation = &self_point.sub(other);
                let mut translated_others: Vec<Point> = Vec::new();
                
                for to_translate in &other_points {
                    translated_others.push(to_translate.add(translation));
                }

                let intersection:Vec<Point> = self.fixed_points.intersect(translated_others);

                let overlapping = intersection.len();

                if overlapping > 1 {
                    //print!("{}|", overlapping)
                    //println!("  Overlapping with {}", overlapping);
                }

                if overlapping >= 12 {
                    return intersection;
                }
            }
        }

        return Vec::new();
    }
}