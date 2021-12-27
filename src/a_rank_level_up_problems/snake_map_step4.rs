use crate::utils;

pub fn main() {
    utils::resolve(SnakeMapStep4Resolver {})
}

#[derive(Debug, PartialEq)]
struct SnakeMapStep4Input {
    map: Vec<String>,
    height: usize,
    width: usize,
}

struct SnakeMapStep4Resolver;

impl SnakeMapStep4Resolver {
    fn find_target_positions(&self, input: SnakeMapStep4Input) -> Vec<String> {
        let mut positions = Vec::new();
        if input.height < 2 {
            return positions;
        }

        // 先頭が対象かチェック
        for x in 0..input.width {
            let char = input.map[1].chars().nth(x).unwrap();
            if self.is_target_char(char) {
                positions.push(self.create_position_string(x, 0));
            }
        }

        // 文字列の中で最初に#が出てくる場所を探し、その2つ下も#だったら、1つ下が対象になる
        for y in 0..input.height - 2 {
            for x in 0..input.width {
                let current_char = input.map[y].chars().nth(x).unwrap();
                let tow_below_char = input.map[y + 2].chars().nth(x).unwrap();
                if self.is_target_char(current_char) && self.is_target_char(tow_below_char) {
                    positions.push(self.create_position_string(x, y + 1));
                }
            }
        }

        // 最終行が対象かチェック
        for x in 0..input.width {
            let char = input.map[input.height - 2].chars().nth(x).unwrap();
            if self.is_target_char(char) {
                positions.push(self.create_position_string(x, input.height - 1));
            }
        }

        return positions;
    }

    fn is_target_char(&self, char: char) -> bool {
        char.eq(&'#')
    }

    fn create_position_string(&self, x: usize, y: usize) -> String {
        format!("{} {}", y, x)
    }
}

impl utils::Resolver for SnakeMapStep4Resolver {
    type Input = SnakeMapStep4Input;

    fn read_input(&self) -> Self::Input {
        let first_line: Vec<usize> = utils::InputReader::read_line_splitted_by_whitespace();
        let map_heigh = first_line[0];
        SnakeMapStep4Input {
            map: utils::InputReader::read_lines(map_heigh),
            height: map_heigh,
            width: first_line[1],
        }
    }

    fn create_output(&self, input: Self::Input) -> String {
        self.find_target_positions(input).join("\n")
    }
}

#[cfg(test)]
mod tests {

    use crate::utils::Resolver;

    use super::*;

    #[test]
    fn test_create_output_1() {
        let input = SnakeMapStep4Input {
            map: vec![
                String::from("..."),
                String::from("..."),
                String::from("..."),
            ],
            height: 3,
            width: 3,
        };
        let resolver = SnakeMapStep4Resolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, String::from(""));
    }

    #[test]
    fn test_create_output_2() {
        let input = SnakeMapStep4Input {
            map: vec![String::from("."), String::from("#")],
            height: 2,
            width: 1,
        };
        let resolver = SnakeMapStep4Resolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, String::from("0 0"));
    }

    #[test]
    fn test_create_output_3() {
        let input = SnakeMapStep4Input {
            map: vec![String::from("#"), String::from(".")],
            height: 2,
            width: 1,
        };
        let resolver = SnakeMapStep4Resolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, String::from("1 0"));
    }

    #[test]
    fn test_create_output_4() {
        let input = SnakeMapStep4Input {
            map: vec![
                String::from("###"),
                String::from("..."),
                String::from("###"),
            ],
            height: 3,
            width: 3,
        };
        let resolver = SnakeMapStep4Resolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, String::from("1 0\n1 1\n1 2"));
    }

    #[test]
    fn test_create_output_5() {
        let input = SnakeMapStep4Input {
            map: vec![
                String::from("#.#."),
                String::from(".#.#"),
                String::from(".#.#"),
                String::from("#.#."),
            ],
            height: 4,
            width: 4,
        };
        let resolver = SnakeMapStep4Resolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, String::from("0 1\n0 3\n3 1\n3 3"));
    }
}
