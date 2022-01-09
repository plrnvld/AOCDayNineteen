use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut scanners = Vec::new();

    read_scanners(&mut scanners);

    println!("Scanners read: {}", scanners.len());

    for s in scanners {
        println!("{:?}", s)
    }
}

fn read_scanners(scanners: &mut Vec<Scanner>) -> () {
    let read_result =  read_lines("src/Input.txt");

    if let Ok(lines) = read_result {
        
        let mut line_num = 0;
        for line_result in lines {
            if let Ok(line) = line_result {
                println!("Parsing line: {}", line);

                let scan_start = "--- scanner ";
                if line.starts_with(scan_start) {

                    let scanner_nums: Vec<&str> = line.split(" ").collect();
                    let scanner_num: u8 = scanner_nums.get(2).unwrap().parse::<u8>().unwrap();
                    let scanner = Scanner::new(scanner_num);
                    scanners.push(scanner);
                }
            }

            line_num += 1;
        }

        println!("Num lines {}", line_num)
    }
    else if let Err(problem) = read_result {
        println!("Problems: {}", problem);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
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
        println!("New called");
            
        return Scanner {
            num: num , 
            points: Vec::new() 
        } 
    }    
}
