pub fn set_number(char: char, first_num: &mut Option<char>, last_num: &mut Option<char>) {
    if first_num.is_none() {
        *first_num = Some(char);
    } else {
        *last_num = Some(char);
    };
}

#[cfg(test)]
mod tests {
    use crate::utils::input_string_from_file;
    use std::num::ParseIntError;
    use std::{fs, path::Path};

    use super::*;

    #[test]
    fn part_1() {
        let input = input_string_from_file(file!());

        let mut count: usize = 0;

        for line in input.lines() {
            let mut first_num: Option<char> = None;
            let mut last_num: Option<char> = None;

            for char in line.chars() {
                match char {
                    c @ '0'..='9' => {
                        if first_num.is_none() {
                            first_num = Some(c);
                        } else {
                            last_num = Some(c);
                        }
                    }
                    _ => {}
                }
            }

            let number: usize = format!(
                "{}{}",
                first_num.expect(line),
                last_num.unwrap_or(first_num.unwrap())
            )
            .parse()
            .unwrap();
            count += number;
        }

        println!("count: {count}")
    }

    #[test]
    fn part_2() {
        let input = input_string_from_file(file!());

        let mut count: usize = 0;

        for line in input.lines() {
            let mut first_num: Option<char> = None;
            let mut last_num: Option<char> = None;

            //
            let mut skip = 0;
            for (index, char) in line.chars().enumerate() {
                // if skip > 0 {
                //     skip -= 1;
                //     continue;
                // }

                let mut set_helper = |val: char| {
                    if first_num.is_none() {
                        first_num = Some(char)
                    } else {
                        last_num = Some(char)
                    }
                };

                match char {
                    c @ '0'..='9' => {
                        set_helper(c);
                    }
                    'z' => {
                        if (&line[index..])
                            .find("zero")
                            .is_some_and(|f_index| f_index == 0)
                        {
                            // set_helper('0');
                            set_number('0', &mut first_num, &mut last_num);

                            skip += 4;
                        }
                    }
                    'o' => {
                        if (&line[index..])
                            .find("one")
                            .is_some_and(|f_index| f_index == 0)
                        {
                            // set_helper('1');
                            set_number('1', &mut first_num, &mut last_num);

                            skip += 3;
                        }
                    }
                    't' => {
                        if (&line[index..])
                            .find("two")
                            .is_some_and(|f_index| f_index == 0)
                        {
                            set_number('2', &mut first_num, &mut last_num);
                            skip += 3;
                        } else if (&line[index..])
                            .find("three")
                            .is_some_and(|f_index| f_index == 0)
                        {
                            set_number('3', &mut first_num, &mut last_num);
                            skip += 5;
                        };
                    }
                    'f' => {
                        if (&line[index..])
                            .find("four")
                            .is_some_and(|f_index| f_index == 0)
                        {
                            set_number('4', &mut first_num, &mut last_num);
                            skip += 4;
                        } else if (&line[index..])
                            .find("five")
                            .is_some_and(|f_index| f_index == 0)
                        {
                            set_number('5', &mut first_num, &mut last_num);
                            skip += 4;
                        };
                    }
                    's' => {
                        if (&line[index..])
                            .find("six")
                            .is_some_and(|f_index| f_index == 0)
                        {
                            set_number('6', &mut first_num, &mut last_num);
                            skip += 3;
                        } else if (&line[index..])
                            .find("seven")
                            .is_some_and(|f_index| f_index == 0)
                        {
                            set_number('7', &mut first_num, &mut last_num);
                            skip += 5;
                        };
                    }
                    'e' => {
                        if (&line[index..])
                            .find("eight")
                            .is_some_and(|f_index| f_index == 0)
                        {
                            set_number('8', &mut first_num, &mut last_num);
                            skip += 5;
                        };
                    }
                    'n' => {
                        if (&line[index..])
                            .find("nine")
                            .is_some_and(|f_index| f_index == 0)
                        {
                            set_number('9', &mut first_num, &mut last_num);
                            skip += 4;
                        }
                    }
                    _ => {}
                }
            }

            //
            let result = format!(
                "{}{}",
                first_num.expect(line),
                last_num.unwrap_or(first_num.unwrap())
            )
            .parse::<usize>();

            let number: usize = match result {
                Ok(num) => num,
                Err(err) => {
                    println!("{line}: {:?} {:?}", first_num, last_num);
                    panic!("{}", err)
                }
            };

            println!("number: {number}");
            count += number;
        }

        println!("count: {count}")
    }

    #[test]
    fn ree() {
        let stri = "123123eightone";

        println!(
            "{}",
            (&stri[11..])
                .find("one")
                .is_some_and(|f_index| f_index == 0)
        );
    }
}
