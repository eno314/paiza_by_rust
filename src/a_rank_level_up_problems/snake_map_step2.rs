use crate::utils;

pub fn main() {
    utils::resolve(SnakeMapStep2Resolver {})
}

#[derive(Debug, PartialEq)]
struct SnakeMapStep2Input {
    map: Vec<String>,
    positions: Vec<(usize, usize)>,
}

struct SnakeMapStep2Resolver;

impl SnakeMapStep2Resolver {
    fn create_input(&self, map: Vec<String>, pos_list: Vec<Vec<usize>>) -> SnakeMapStep2Input {
        let positions = pos_list.iter().map(|pos| (pos[0], pos[1])).collect();
        SnakeMapStep2Input { map, positions }
    }

    fn replace_char_of_string(&self, str: &mut String, index: usize) {
        str.replace_range(index..index + 1, "#")
    }
}

impl utils::Resolver for SnakeMapStep2Resolver {
    type Input = SnakeMapStep2Input;

    fn read_input(&self) -> Self::Input {
        let first_line: Vec<usize> = utils::InputReader::read_line_splitted_by_whitespace();
        let (map_heigh, answer_count) = (first_line[0], first_line[2]);
        self.create_input(
            utils::InputReader::read_lines(map_heigh),
            utils::InputReader::read_lines_splitted_by_whitespace(answer_count),
        )
    }

    fn create_output(&self, input: Self::Input) -> String {
        let mut new_map = input.map;
        input
            .positions
            .iter()
            .for_each(|(x, y)| self.replace_char_of_string(&mut new_map[*x], *y));
        new_map.join("\n")
    }
}

#[cfg(test)]
mod tests {

    use crate::utils::Resolver;

    use super::*;

    #[test]
    fn test_create_output_1() {
        let input = SnakeMapStep2Input {
            map: vec![
                String::from("..."),
                String::from("..."),
                String::from("..."),
            ],
            positions: vec![(0, 0)],
        };
        let resolver = SnakeMapStep2Resolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, String::from("#..\n...\n..."));
    }

    #[test]
    fn test_create_output_2() {
        let input = SnakeMapStep2Input {
            map: vec![
                String::from("####"),
                String::from("####"),
                String::from("...."),
                String::from("##.."),
            ],
            positions: vec![(2, 0), (2, 2)],
        };
        let resolver = SnakeMapStep2Resolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, String::from("####\n####\n#.#.\n##.."));
    }
}
