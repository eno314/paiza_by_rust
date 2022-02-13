use crate::utils;

pub fn main() {
    utils::resolve(SnakeMapStep1Resolver {})
}

#[derive(Debug, PartialEq)]
struct SnakeMapStep1Input {
    map: Vec<String>,
    positions: Vec<(usize, usize)>,
}

struct SnakeMapStep1Resolver;

impl SnakeMapStep1Resolver {
    fn create_input(&self, map: Vec<String>, pos_list: Vec<Vec<usize>>) -> SnakeMapStep1Input {
        let positions = pos_list.iter().map(|pos| (pos[0], pos[1])).collect();
        SnakeMapStep1Input { map, positions }
    }
}

impl utils::Resolver for SnakeMapStep1Resolver {
    type Input = SnakeMapStep1Input;

    fn read_input(&self) -> Self::Input {
        let first_line: Vec<usize> = self.read_line_splitted_by_whitespace();
        let (map_heigh, answer_count) = (first_line[0], first_line[2]);
        self.create_input(
            self.read_lines(map_heigh),
            self.read_lines_splitted_by_whitespace(answer_count),
        )
    }

    fn create_output(&self, input: Self::Input) -> String {
        input
            .positions
            .iter()
            .map(|(x, y)| input.map[*x].chars().nth(*y).unwrap().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {

    use crate::utils::Resolver;

    use super::*;

    #[test]
    fn test_create_input_1() {
        let map = vec![
            String::from("###"),
            String::from("###"),
            String::from("..."),
        ];
        let pos_list = vec![vec![2, 2], vec![1, 1]];
        let resolver = SnakeMapStep1Resolver {};

        let actual = resolver.create_input(map, pos_list);

        let expected = SnakeMapStep1Input {
            map: vec![
                String::from("###"),
                String::from("###"),
                String::from("..."),
            ],
            positions: vec![(2, 2), (1, 1)],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_input_2() {
        let map = vec![String::from("#."), String::from(".#")];
        let pos_list = vec![vec![2, 2], vec![1, 1]];
        let resolver = SnakeMapStep1Resolver {};

        let actual = resolver.create_input(map, pos_list);

        let expected = SnakeMapStep1Input {
            map: vec![String::from("#."), String::from(".#")],
            positions: vec![(2, 2), (1, 1)],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_output_1() {
        let input = SnakeMapStep1Input {
            map: vec![
                String::from("###"),
                String::from("###"),
                String::from("..."),
            ],
            positions: vec![(2, 2), (1, 1)],
        };
        let resolver = SnakeMapStep1Resolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, String::from(".\n#"));
    }

    #[test]
    fn test_create_output_2() {
        let input = SnakeMapStep1Input {
            map: vec![String::from("#."), String::from(".#")],
            positions: vec![(0, 1)],
        };
        let resolver = SnakeMapStep1Resolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, String::from("."));
    }
}
