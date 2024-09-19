use crate::shared::DayResult;
use std::fs;

pub fn solve() -> Result<DayResult, String> {
    let input = match fs::read_to_string("src/day_6/input.txt") {
        Ok(result) => result,
        Err(err) => return Err(format!("Error in reading file: {}", err)),
    };
    let output = DayResult {
        part_1: solve_part_1(&input)?,
        part_2: solve_part_2(&input)?,
    };

    return Ok(output);
}

fn solve_part_1(_input: &str) -> Result<usize, String> {
    let input = vec![
        RaceInfo {
            time: 61,
            distance_to_beat: 643,
        },
        RaceInfo {
            time: 70,
            distance_to_beat: 1184,
        },
        RaceInfo {
            time: 90,
            distance_to_beat: 1362,
        },
        RaceInfo {
            time: 66,
            distance_to_beat: 1041,
        },
    ];
    let solution_counts = input
        .iter()
        .map(|race_info| solutions_to_race_info(race_info))
        .collect::<Result<Vec<usize>, String>>()?;
    let output = solution_counts.iter().fold(1, |product, next| product * next);
    return Ok(output)
}

fn solve_part_2(input: &str) -> Result<usize, String> {
    let one_big_race = RaceInfo {
        time: 61709066,
        distance_to_beat: 643118413621041,
    };
    let solution_count = solutions_to_race_info(&one_big_race)?;

    return Ok(solution_count);
}

struct RaceInfo {
    time: usize,
    distance_to_beat: usize,
}

fn solutions_to_race_info(race_info: &RaceInfo) -> Result<usize, String> {
    let solutions = solve_quadratic(race_info.time, race_info.distance_to_beat)?;
    let output = (solutions.1 as usize)
        .checked_sub(solutions.0 as usize)
        .ok_or("No solutions found")?;
    Ok(output)
}

fn solve_quadratic(b: usize, c: usize) -> Result<(f64, f64), String> {
    // solve (T-x)x - D = 0
    // Tx - x^2 - D
    let discriminant = (b * b)
        .checked_sub(4 * c)
        .ok_or("Discriminant less than 0")?;
    let small_solution = ((b as f64) - (discriminant as f64).sqrt()) / 2.0;
    let large_solution = ((b as f64) + (discriminant as f64).sqrt()) / 2.0;
    Ok((small_solution, large_solution))
}

#[cfg(test)]
mod tests {
    use crate::day_6::solutions_to_race_info;

    use super::RaceInfo;

    #[test]
    fn solutions_to_race_info_solves_example_1() {
        let info = RaceInfo {
            time: 7,
            distance_to_beat: 9,
        };

        let output = match solutions_to_race_info(&info) {
            Ok(result) => result,
            Err(e) => panic!("Error in solutions_to_race_info: {}", e),
        };

        assert_eq!(output, 4)
    }
}
