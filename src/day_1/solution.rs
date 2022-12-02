use crate::common::read_file_to_string;

pub fn part_1() -> i32 {
    let input = read_file_to_string("./src/day_1/part_1_input.txt");
    let groups = input.split("\n\n");

    let largest_sum = groups
        .map(|x| -> i32 {
            x.split("\n")
                .filter(|l| !l.is_empty())
                .map(|num| num.parse::<i32>().unwrap())
                .sum()
        })
        .max();

    largest_sum.unwrap()
}

pub fn part_2() -> i32 {
    let input = read_file_to_string("./src/day_1/part_2_input.txt");
    let groups = input.split("\n\n");

    let mut sums = groups
        .map(|x| -> i32 {
            x.split("\n")
                .filter(|l| !l.is_empty())
                .map(|num| num.parse::<i32>().unwrap())
                .sum()
        })
        .collect::<Vec<i32>>();
    sums.sort_by(|a, b| b.cmp(a));

    sums.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_solution() {
        assert_eq!(part_1(), 71471);
    }

    #[test]
    fn part_2_solution() {
        assert_eq!(part_2(), 211189);
    }
}
