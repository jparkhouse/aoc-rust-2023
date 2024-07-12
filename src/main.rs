use std::error::Error;
use std::io;

mod shared;
mod day_1;
mod day_2;
mod day_3;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter day number: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let day: usize = input.trim().parse()?;

    run_day_solution(day);

    Ok(())
}

fn run_day_solution(day: usize) {
    if day < 1 || day > 25 {
        println!("{} is an invalid day for advent.", day);
        return;
    }

    match day {
        1 => match day_1::solve() {
            Ok(result) => println!(
                "Day {} solution:\nPart 1: {}\nPart 2: {}",
                day, result.0, result.1
            ),
            Err(err) => eprintln!("Error in day {} solution: {}", day, err),
        },
        2 => match day_2::solve() {
            Ok(result) => println!(
                "Day {} solution:\nPart 1: {}\nPart 2: {}",
                day, result.0, result.1
            ),
            Err(err) => eprintln!("Error in day {} solution: {}", day, err),
        },
        3 => match day_3::solve() {
            Ok(result) => println!(
                "Day {} solution:\nPart 1: {}\nPart 2: {}",
                day, result.0, result.1
            ),
            Err(err) => eprintln!("Error in day {} solution: {}", day, err),
        },
        _ => println!("Solution for day {} not implemented yet", day),
    }
}
