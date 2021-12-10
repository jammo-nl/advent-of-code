use crate::tools::read_lines;

pub fn run_all(input: &str) -> String {
    let part1 = part1(read_lines(input));
    let part2 = part2(read_lines(input));

    format!("
Result part 1: {}
Result part 2: {}
    ", part1, part2)
}

pub fn process_parts(code: &mut Vec<char>) -> Result<Vec<char>, Option<(char,char)>> {
    let mut my_open: Option<char> = None;
    let mut result: Vec<char> = vec!();

    while !code.is_empty() {
        // get the first character of the code
        let data = *code.first().unwrap();

        // check if we have an open or close tag
        if ['(','[','{','<'].contains(&data) {
            // if it is an open tag and we have an open tag, call recursive, else close this tag and continue
            match my_open {
                Some(_) => {
                    match process_parts(code) {
                        Ok(mut data) => {
                            result.append(&mut data);
                        }
                        Err(err_item) => {
                            match err_item {
                                Some(item) => { return Err(Some(item))},
                                None => {
                                    if !code.is_empty() {
                                        // update data so we look at the correct item..
                                        let data = *code.first().unwrap();
                                        match my_open {
                                            Some(item) => {
                                                if (item == '(' && data == ')')
                                                    || (item == '[' && data == ']')
                                                    || (item == '{' && data == '}')
                                                    || (item == '<' && data == '>') {
                                                        my_open = None;
                                                        code.remove(0);
                                                    }
                                                else {
                                                    match item {
                                                        '(' => {return Err(Some((')', data)))},
                                                        '[' => {return Err(Some((']', data)))},
                                                        '{' => {return Err(Some(('}', data)))},
                                                        '<' => {return Err(Some(('>', data)))},
                                                        _ => {return Err(None)}
                                                    }
                                                }
                                            },
                                            None => {
                                                return Err(None);
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                None => {
                    my_open = Some(data);
                    code.remove(0);
                },
            }

        } else if [')',']','}','>'].contains(&data) {
            // if it is a close tag and we have an open tag, check if we can close this.
            match my_open {
                Some(item) => {
                    if (item == '(' && data == ')')
                        || (item == '[' && data == ']')
                        || (item == '{' && data == '}')
                        || (item == '<' && data == '>') {
                            my_open = None;
                            code.remove(0);
                        }
                    else {
                        match item {
                            '(' => {return Err(Some((')', data)))},
                            '[' => {return Err(Some((']', data)))},
                            '{' => {return Err(Some(('}', data)))},
                            '<' => {return Err(Some(('>', data)))},
                            _ => {return Err(None)}
                        }
                    }
                },
                None => {
                    return Err(None);
                },
            }
        } else {
            panic!("unknown tag!!");
        }
    }

    if let Some(open_item) = my_open {
        if !result.is_empty() && open_item == *result.first().unwrap() {
            result.remove(0);
        }
        else {
            match open_item {
                '(' => {result.push(')');},
                '[' => {result.push(']');},
                '{' => {result.push('}');},
                '<' => {result.push('>');},
                _ => {}
            }
        }
    }
    Ok(result)
}

pub fn part1(input: Vec<String>) -> usize {
    let mut syntax_errors: Vec<char> = vec!();
    for line in input {
        match process_parts(&mut line.chars().collect()) {
            Err(item) => {syntax_errors.push(item.unwrap().1)},
            _ => {}
        }
    }

    let mut result = 0;
    result += syntax_errors.iter().filter(|i| *i == &')').count()*3;
    result += syntax_errors.iter().filter(|i| *i == &']').count()*57;
    result += syntax_errors.iter().filter(|i| *i == &'}').count()*1197;
    result += syntax_errors.iter().filter(|i| *i == &'>').count()*25137;

    result
}
pub fn part2(input: Vec<String>) -> usize {
    let mut scores: Vec<usize> = vec!();
    for line in input {
        match process_parts(&mut line.chars().collect()) {
            Ok(missing) => {
                let mut result = 0;
                for item in &missing {
                    result *= 5;
                    match item {
                        ')' => {result += 1},
                        ']' => {result += 2},
                        '}' => {result += 3},
                        '>' => {result += 4},
                        _ => {}
                    }
                }
                scores.push(result);
            },
            _ => {}
        }
    }

    scores.sort_unstable();
    scores[scores.len()/2]
}

#[cfg(test)]
mod tests {
    static INPUT: &str = "./inputs/day10_test.txt";
    
    use super::*;
    #[test]
    fn test_part1() {
        let output = part1(read_lines(INPUT));
        assert_eq!(output, 26397)
    }

    #[test]
    fn test_part2() {
        let output = part2(read_lines(INPUT));
        assert_eq!(output, 288957)
    }
}
