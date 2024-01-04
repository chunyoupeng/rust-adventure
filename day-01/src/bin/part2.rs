use std::fs::File;
use std::io::{BufRead, BufReader, self};

fn main() -> io::Result<()>{
    let rt = part1("src/bin/input.txt");
    println!("Part1 answer is {}", rt);

    Ok(())
}


fn part1(input: &str) -> u32 {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let digits_v = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut rt: u32 = 0;
    
    for lin in reader.lines() {
        let line = lin.unwrap();
        let mut first_num = None;
        let mut last_num = None;

        for ch in line.chars() {
            if ch.is_digit(10){
                last_num = Some( ch.to_digit(10).unwrap() );
                if first_num.is_none(){
                    first_num = last_num;
                }
            }
        }
        if let (Some(first), Some(last)) = (first_num, last_num){
            rt += first* 10 + last;
        }
    }
    rt
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(142, part1("src/bin/test.txt"));
    }
}