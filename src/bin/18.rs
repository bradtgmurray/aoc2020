use std::str::Chars;
use aoc2020::utils;

fn eval_value(chars: &mut Chars) -> usize {
    let vc = chars.next().unwrap();

    return if vc == '(' {
        // left side is a sub-expression
        eval_expr(chars)
    } else {
        vc.to_digit(10).unwrap() as usize
    };
}

fn eval_expr(chars: &mut Chars) -> usize {
    let mut v = eval_value(chars);

    chars.next(); // skip space

    loop {
        let op = chars.next().unwrap(); // skip space
        chars.next(); // skip space

        let r: usize = eval_value(chars);

        if op == '+' {
            v = v + r;
        } else {
            v = v * r;
        }

        let p = chars.next();
        if p.is_none() || p.unwrap() == ')' {
            return v;
        }

        // Else it's a space, we're about to get another op
    }
}

fn evaluate(s: &str) -> usize {
    eval_expr(&mut s.chars())
}

fn eval_value_part2(chars: &mut Chars) -> usize {
    let vc = chars.next().unwrap();

    return if vc == '(' {
        // left side is a sub-expression
        let v = eval_expr_part2(chars);

        // Consume closing brace
        chars.next();

        v
    } else {
        vc.to_digit(10).unwrap() as usize
    };
}

fn peek_next(chars: &Chars) -> Option<char> {
    let mut copy = chars.clone();
    copy.next()
}

fn eval_expr_part2(chars: &mut Chars) -> usize {
    let mut v = eval_value_part2(chars);

    loop {
        let p = peek_next(chars);
        if p.is_none() || p.unwrap() == ')' {
            return v;
        }

        chars.next(); // skip space
        let op = chars.next().unwrap();
        chars.next(); // skip space

        if op == '+' {
            // We do adds first, so just consume the value and do the math
            v = v + eval_value_part2(chars);
        } else {
            // We do mults last, so evaluate the whole expression to the right before doing this
            // multiplication
            v = v * eval_expr_part2(chars);
        }
    }
}

fn part2(s: &str) -> usize {
    eval_expr_part2(&mut s.chars())
}

fn main() {
    let data: Vec<String> = utils::read_lines("./input_data/18.txt");

    println!("Part 1: {}", data.iter().map(|l| evaluate(l)).sum::<usize>());
    println!("Part 2: {}", data.iter().map(|l| part2(l)).sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_simple() {
        assert_eq!(evaluate("1 + 2"), 3);
    }

    #[test]
    fn test_example_1_part1() {
        assert_eq!(evaluate("1 + 2 * 3 + 4 * 5 + 6"), 71);
    }

    #[test]
    fn test_example_5_part1() {
        assert_eq!(evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
    }

    #[test]
    fn test_part2_example_simple() {
        assert_eq!(part2("1 + 2"), 3);
        assert_eq!(part2("2 * 3"), 6);
        assert_eq!(part2("1 + 2 * 3"), 9);
        assert_eq!(part2("1 + 2 * 3 + 4"), 21);
        assert_eq!(part2("1 + 2 * 3 * 4"), 36);
    }

    #[test]
    fn test_example_2_part2() {
        assert_eq!(part2("2 * 3 + (4 * 5)"), 46);
    }

    #[test]
    fn test_example_3_part2() {
        assert_eq!(part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    }

    #[test]
    fn test_example_5_part2() {
        assert_eq!(part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
    }
}