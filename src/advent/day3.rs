use super::file;
use ndarray;

pub(crate) fn main() {
    println!("Test");
    let file = file::file_read("data/3.txt");
    let lines = file.lines();

    let mut fabric = ndarray::Array2::<u64>::zeros((1000,1000));
    for line in lines {
        let first_split: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        let coords: Vec<String> = first_split[2].split(',').map(|s| s.to_string()).collect();
        let size: Vec<String> = first_split[3].split('x').map(|s| s.to_string()).collect();
        let x1 = coords[0].parse::<usize>().unwrap();
        let y1 = coords[1][0..coords[1].len()-1].parse::<usize>().unwrap();
        let x2 = x1 + size[0].parse::<usize>().unwrap();
        let y2 = y1 + size[1].parse::<usize>().unwrap();


        for x in x1..x2 {
            for y in y1..y2 {
                fabric[[x, y]] += 1;
            }
        }
    }

    let result = fabric.outer_iter().flatten().filter(|x| **x > 1).count();
    println!("{}", result);


    
}
