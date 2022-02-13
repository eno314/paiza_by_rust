use crate::utils;

pub fn main() {
    utils::resolve(SnakeMapStep3Resolver {})
}

#[derive(Debug, PartialEq)]
struct SnakeMapStep3Input {
    map: Vec<String>,
}

struct SnakeMapStep3Resolver;

impl SnakeMapStep3Resolver {
    fn find_target_positions(&self, y_pos: usize, line: &String) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();

        let chars: Vec<char> = line.chars().collect();
        let chars_len = chars.len();
        if chars_len < 2 {
            return positions;
        }

        // 先頭が対象かチェック
        if self.is_target_char(chars[1]) {
            positions.push((0, y_pos));
        }

        // 文字列の中で最初に#が出てくる場所を探し、その2つ右も#だったら、1つ右が対象になる
        for x in 0..(chars.len() - 2) {
            if self.is_target_char(chars[x]) {
                if self.is_target_char(chars[x + 2]) {
                    positions.push((x + 1, y_pos));
                }
            }
        }

        // 最後が対象かチェック
        if self.is_target_char(chars[chars_len - 2]) {
            positions.push((chars_len - 1, y_pos));
        }

        return positions;
    }

    fn is_target_char(&self, char: char) -> bool {
        char.eq(&'#')
    }
}

impl utils::Resolver for SnakeMapStep3Resolver {
    type Input = SnakeMapStep3Input;

    fn read_input(&self) -> Self::Input {
        let first_line: Vec<usize> = self.read_line_splitted_by_whitespace();
        let map_heigh = first_line[0];
        SnakeMapStep3Input {
            map: self.read_lines(map_heigh),
        }
    }

    fn create_output(&self, input: Self::Input) -> String {
        input
            .map
            .iter()
            .enumerate()
            .flat_map(|(y_pos, line)| self.find_target_positions(y_pos, line))
            .map(|(x, y)| format!("{} {}", y, x))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {

    use crate::utils::Resolver;

    use super::*;

    #[test]
    fn test_create_output_1() {
        let input = SnakeMapStep3Input {
            map: vec![
                String::from("..."),
                String::from("..."),
                String::from("..."),
            ],
        };
        let resolver = SnakeMapStep3Resolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, String::from(""));
    }

    #[test]
    fn test_create_output_2() {
        let input = SnakeMapStep3Input {
            map: vec![String::from("#.#")],
        };
        let resolver = SnakeMapStep3Resolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, String::from("0 1"));
    }

    #[test]
    fn test_create_output_3() {
        let input = SnakeMapStep3Input {
            map: vec![String::from(".#.")],
        };
        let resolver = SnakeMapStep3Resolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, String::from("0 0\n0 2"));
    }

    #[test]
    fn test_create_output_4() {
        let input = SnakeMapStep3Input {
            map: vec![
                String::from("#.#"),
                String::from(".#."),
                String::from("..."),
            ],
        };
        let resolver = SnakeMapStep3Resolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, String::from("0 1\n1 0\n1 2"));
    }

    #[test]
    fn test_create_output_5() {
        let input = SnakeMapStep3Input {
            map: vec![
                String::from("####"),
                String::from("####"),
                String::from("####"),
                String::from("####"),
            ],
        };
        let resolver = SnakeMapStep3Resolver {};

        let actual = resolver.create_output(input);

        assert_eq!(
            actual,
            String::from(
                "0 0\n0 1\n0 2\n0 3\n1 0\n1 1\n1 2\n1 3\n2 0\n2 1\n2 2\n2 3\n3 0\n3 1\n3 2\n3 3"
            )
        );
    }

    #[test]
    fn test_create_output_6() {
        let input = SnakeMapStep3Input {
            map: vec![
                String::from("##"),
                String::from(".."),
                String::from("#."),
                String::from(".#"),
            ],
        };
        let resolver = SnakeMapStep3Resolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, String::from("0 0\n0 1\n2 1\n3 0"));
    }
}
