use crate::utils;

pub fn main() {
    utils::resolve(SnakeMoveStep1Resolver {})
}

struct SnakeMoveStep1Input {
    map: Vec<String>,
    height: usize,
    width: usize,
}

struct SnakeMoveStep1Resolver;

impl SnakeMoveStep1Resolver {
    fn find_first_target_position(&self, input: SnakeMoveStep1Input) -> (usize, usize) {
        for y in 0..input.height {
            for x in 0..input.width {
                if self.is_target_position(x, y, &input.map) {
                    return (x, y);
                }
            }
        }
        panic!("illegal input")
    }

    fn is_target_position(&self, x: usize, y: usize, map: &Vec<String>) -> bool {
        self.is_target_char(map[y].chars().nth(x).unwrap())
    }

    fn is_target_char(&self, char: char) -> bool {
        char.eq(&'#')
    }
}

impl utils::Resolver for SnakeMoveStep1Resolver {

    type Input = SnakeMoveStep1Input;

    fn read_input(&self) -> Self::Input {
        let first_line: Vec<usize> = utils::InputReader::read_line_splitted_by_whitespace();
        let map_heigh = first_line[0];
        SnakeMoveStep1Input {
            map: utils::InputReader::read_lines(map_heigh),
            height: map_heigh,
            width: first_line[1],
        }
    }

    fn create_output(&self, input: Self::Input) -> String {
        let (x, y) = self.find_first_target_position(input);
        format!("{} {}", y, x)
    }
}


#[cfg(test)]
mod output_tests {

    use crate::utils::Resolver;

    use super::*;

    #[test]
    fn test_create_output_1() {
        let input = SnakeMoveStep1Input {
            map: vec![
                String::from("#"),
            ],
            height: 1,
            width: 1,
        };
        let resolver = SnakeMoveStep1Resolver{};

        let actual = resolver.create_output(input);

        assert_eq!(actual, String::from("0 0"))
    }

    #[test]
    fn test_create_output_2() {
        let input = SnakeMoveStep1Input {
            map: vec![
                String::from(".#."),
                String::from("..."),
                String::from("..."),
            ],
            height: 3,
            width: 3,
        };
        let resolver = SnakeMoveStep1Resolver{};

        let actual = resolver.create_output(input);

        assert_eq!(actual, String::from("0 1"))
    }
}
