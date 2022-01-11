use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let scanners = read_scanners();

    println!("Scanners read: {}", scanners.len());

    let point = Point{ x:4, y:1, z: 9 };


    let all_rotations = point.all_point_rotations();

    println!("Rotations");

    for rot in all_rotations {
        println!("{:?}", rot);
    }
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

#[derive(Debug, Clone, Copy, Default)]
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
        let base_rotations: [fn(&Point) -> Point; 4] = [
            |p| p.clone(),
            |p| p.rot_along_x(),
            |p| p.rot_along_x().rot_along_x(),
            |p| p.rot_along_x().rot_along_x().rot_along_x()
        ];

        let secondary_rotations: [fn(Point) -> Point; 6] = [
            |p| p.clone(),
            |p| p.rot_along_z(),
            |p| p.rot_along_z().rot_along_z(),
            |p| p.rot_along_z().rot_along_z().rot_along_z(),
            |p| p.rot_along_y(),
            |p| p.rot_along_y().rot_along_y().rot_along_y(),
        ];

        let mut result_array: [Point; 24] = Default::default();

        let mut index = 0;
        for f1 in base_rotations {
            for f2 in secondary_rotations {
                result_array[index] = f2(f1(&self));
                index += 1;
            }
        }

        return result_array;
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
