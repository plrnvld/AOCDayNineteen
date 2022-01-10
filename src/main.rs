use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let scanners = read_scanners();

    println!("Scanners read: {}", scanners.len());
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


#[derive(Debug, Clone, Copy)]
struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point {
    fn rot_x(&self) -> Point {
        let new_point = Point { x: 1, y: 2, z: 3 };

        return new_point;
    }
}

#[derive(Debug)]
struct Scanner {
    pub num: u8,
    pub points: Vec<Point>
}

impl Scanner {
    pub fn new(num: u8) -> Scanner { 
        return Scanner {
            num: num , 
            points: Vec::new() 
        } 
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(Point { x: point.x, y: point.y, z: point.z })
    }    
}
