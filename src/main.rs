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

static EMPTY_POINT: Point = Point { x:0, y: 0, z:0 }; 

fn main() {
    let mut scanners = read_scanners();

    println!("Scanners read: {}", scanners.len());

    let mut fixed_scanners: Vec<Scanner> = vec!(scanners.pop().unwrap());
    
    let mut unfixed_scanners: Vec<Scanner> = Vec::from(scanners).to_vec();
    
    while &unfixed_scanners.len() > &0 {
        for f in fixed_scanners.to_vec() {
            unfixed_scanners.retain(|u| {
                let (matched, scanner_center, scanner_points) = scanners_overlap(u, &f);
                if matched {
                    let scanner_to_fix = Scanner { 
                        num: u.num, 
                        points: scanner_points,
                        location: Some(scanner_center)
                    };

                    fixed_scanners.push(scanner_to_fix);
                }

                return !matched;
            });
        }

        fixed_scanners.remove(0);
    }

    println!("Finished: all fixed points!");
}

fn scanners_overlap(s1:&Scanner, s2: &Scanner) -> (bool, Point, Vec<Point>) {
    println!("Comparing {} with {}", s1.num, s2.num);
        
    //for rot1 in 0..24 {
        let rotated_points1 = &s1.points;
        //let rotated_points1 = get_rotated_points(rot1, &s1.points);

        for rot2 in 0..24 {
            let rotated_points2 = get_rotated_points(rot2, &s2.points);

            let (overlapping, scanner_center) = points_overlap(&rotated_points1, &rotated_points2);
            if overlapping {
                println!("  Matching!");
                return (true, scanner_center, rotated_points2);
            }
        }
    //}             

    return (false, EMPTY_POINT, Vec::new());
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

    fn neg(&self) -> Point {
        return Point { x: -self.x, y: -self.y, z: -self.z };
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

fn points_overlap(points1: &Vec<Point>, points2: &Vec<Point>) -> (bool, Point) {
    for p1 in points1 {
        for p2 in points2 {
            let translation = &p1.sub(&p2);

            let mut translated_p2s: Vec<Point> = Vec::new();
            for to_translate in points2 {
                translated_p2s.push(to_translate.add(translation));
            }

            let intersection:Vec<Point> = points1.intersect(translated_p2s);

            let overlapping = intersection.len();

            if overlapping > 1 {
                //print!("{}|", overlapping)
                //println!("  Overlapping with {}", overlapping);
            }

            if overlapping >= 12 {
                let scanner_center = p1.add(&p2.neg());
                return (true, scanner_center);
            }
        }
    }
    
    return (false, EMPTY_POINT) ;
}


#[derive(Debug, Clone)]
struct Scanner {
    pub num: u8,
    pub points: Vec<Point>,
    pub location: Option<Point>
}

impl Scanner {
    pub fn new(num: u8) -> Scanner { 
        return Scanner {
            num, 
            points: Vec::new(),
            location: None
        } 
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(Point { x: point.x, y: point.y, z: point.z })
    }
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