use regex::Regex;

advent_of_code::solution!(3);

enum Instruction {
    Do,
    DoNot,
    Mul(u32, u32),
}

fn parse(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"(mul|do|don't)\((\d{1,3},\d{1,3})?\)").unwrap();

    re.captures_iter(input)
        .map(|c| {
            let name = c.get(1).unwrap().as_str();
            match name {
                "do" => Instruction::Do,
                "don't" => Instruction::DoNot,
                "mul" => {
                    let params = c.get(2).unwrap().as_str();
                    let (lhs, rhs) = params.split_once(',').unwrap();
                    let lhs = lhs.parse().unwrap();
                    let rhs = rhs.parse().unwrap();
                    Instruction::Mul(lhs, rhs)
                }
                _ => unimplemented!(),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum_product = 0;

    for instr in parse(input).iter() {
        if let Instruction::Mul(lhs, rhs) = instr {
            sum_product += lhs * rhs;
        }
    }

    Some(sum_product)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut enabled = true;
    let mut sum_product = 0;

    for instr in parse(input).iter() {
        match instr {
            Instruction::Do => enabled = true,
            Instruction::DoNot => enabled = false,
            Instruction::Mul(lhs, rhs) => {
                if enabled {
                    sum_product += lhs * rhs;
                }
            }
        }
    }

    Some(sum_product)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        const INPUT: &str =
            r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = part_one(INPUT);
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str =
            r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = part_two(INPUT);
        assert_eq!(result, Some(48));
    }
}
