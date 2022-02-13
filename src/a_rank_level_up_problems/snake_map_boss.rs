use crate::utils;

pub fn main() {
    utils::resolve(SnakeMapBossResolver {})
}

#[derive(Debug, PartialEq)]
struct SnakeMapBossInput {
    map: Vec<String>,
    height: usize,
    width: usize,
}

impl SnakeMapBossInput {
    fn find_target_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_target_position(x, y) {
                    positions.push((x, y))
                }
            }
        }
        positions
    }

    fn is_target_position(&self, x: usize, y: usize) -> bool {
        let check_points = self.create_check_points_of_target(x, y);
        if check_points.len() < 2 {
            return false;
        }

        check_points
            .iter()
            .filter_map(|(target_x, target_y)| self.map[*target_y].chars().nth(*target_x))
            .all(|char| self.is_target_char(char))
    }

    fn create_check_points_of_target(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut check_points = Vec::new();
        if x > 0 {
            check_points.push((x - 1, y));
        }
        if x < self.width - 1 {
            check_points.push((x + 1, y));
        }
        if y > 0 {
            check_points.push((x, y - 1))
        }
        if y < self.height - 1 {
            check_points.push((x, y + 1))
        }
        return check_points;
    }

    fn is_target_char(&self, char: char) -> bool {
        char.eq(&'#')
    }
}

struct SnakeMapBossResolver;

impl utils::Resolver for SnakeMapBossResolver {
    type Input = SnakeMapBossInput;

    fn read_input(&self) -> Self::Input {
        let first_line: Vec<usize> = self.read_line_splitted_by_whitespace();
        let map_heigh = first_line[0];
        SnakeMapBossInput {
            map: self.read_lines(map_heigh),
            height: map_heigh,
            width: first_line[1],
        }
    }

    fn create_output(&self, input: Self::Input) -> String {
        input
            .find_target_positions()
            .iter()
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
        let input = SnakeMapBossInput {
            map: vec![
                String::from("##."),
                String::from("###"),
                String::from("..."),
            ],
            height: 3,
            width: 3,
        };
        let resolver = SnakeMapBossResolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, String::from("0 0\n0 2"));
    }

    #[test]
    fn test_create_output_2() {
        let input = SnakeMapBossInput {
            map: vec![
                String::from("##########"),
                String::from(".........."),
                String::from("##########"),
                String::from("##########"),
                String::from(".........."),
                String::from("#.#.#.#.#."),
                String::from(".#.#.#.#.#"),
                String::from("#.#.#.#.#."),
                String::from(".#.#.#.#.#"),
                String::from(".........."),
            ],
            height: 10,
            width: 10,
        };
        let resolver = SnakeMapBossResolver {};

        let actual = resolver.create_output(input);

        assert_eq!(
            actual,
            String::from("6 0\n6 2\n6 4\n6 6\n6 8\n7 1\n7 3\n7 5\n7 7\n7 9")
        );
    }
}

#[cfg(test)]
mod input_tests {

    use super::SnakeMapBossInput;

    #[test]
    fn is_target_position_when_top_and_left_valid() {
        let input = create_input(&vec![String::from(".#"), String::from("#.")]);
        let actual = input.is_target_position(0, 0);
        assert_eq!(actual, true);
    }

    #[test]
    fn is_target_position_when_top_and_left_invalid() {
        let maps = vec![
            vec![String::from(".#"), String::from("..")],
            vec![String::from(".."), String::from("#.")],
        ];
        for map in maps.iter() {
            let input = create_input(map);
            let actual = input.is_target_position(0, 0);
            assert_eq!(actual, false);
        }
    }

    #[test]
    fn is_target_position_when_bottom_and_right_valid() {
        let input = create_input(&vec![String::from(".#"), String::from("#.")]);
        let actual = input.is_target_position(1, 1);
        assert_eq!(actual, true);
    }

    #[test]
    fn is_target_position_when_bottom_and_right_invalid() {
        let maps = vec![
            vec![String::from(".."), String::from("#.")],
            vec![String::from(".#"), String::from("..")],
        ];
        for map in maps.iter() {
            let input = create_input(map);
            let actual = input.is_target_position(1, 1);
            assert_eq!(actual, false);
        }
    }

    #[test]
    fn is_target_position_when_top_valid() {
        let input = create_input(&vec![String::from("#.#"), String::from(".#.")]);
        let actual = input.is_target_position(1, 0);
        assert_eq!(actual, true);
    }

    #[test]
    fn is_target_position_when_top_invalid() {
        let maps = vec![
            vec![String::from("..#"), String::from(".#.")],
            vec![String::from("#.."), String::from(".#.")],
            vec![String::from("#.#"), String::from("...")],
        ];
        for map in maps.iter() {
            let input = create_input(map);
            let actual = input.is_target_position(1, 0);
            assert_eq!(actual, false);
        }
    }

    #[test]
    fn is_target_position_when_bottom_valid() {
        let input = create_input(&vec![String::from(".#."), String::from("#.#")]);
        let actual = input.is_target_position(1, 1);
        assert_eq!(actual, true);
    }

    #[test]
    fn is_target_position_when_bottom_invalid() {
        let maps = vec![
            vec![String::from(".#."), String::from("..#")],
            vec![String::from(".#."), String::from("#..")],
            vec![String::from("..."), String::from("#.#")],
        ];
        for map in maps.iter() {
            let input = create_input(map);
            let actual = input.is_target_position(1, 1);
            assert_eq!(actual, false);
        }
    }

    #[test]
    fn is_target_position_when_center_valid() {
        let input = create_input(&vec![
            String::from(".#."),
            String::from("#.#"),
            String::from(".#."),
        ]);
        let actual = input.is_target_position(1, 1);
        assert_eq!(actual, true);
    }

    #[test]
    fn is_target_position_when_center_invalid() {
        let maps = vec![
            vec![
                String::from(".#."),
                String::from("..#"),
                String::from(".#."),
            ],
            vec![
                String::from(".#."),
                String::from("#.."),
                String::from(".#."),
            ],
            vec![
                String::from("..."),
                String::from("#.#"),
                String::from(".#."),
            ],
            vec![
                String::from(".#."),
                String::from("#.#"),
                String::from("..."),
            ],
        ];
        for map in maps.iter() {
            let input = create_input(map);
            let actual = input.is_target_position(1, 1);
            assert_eq!(actual, false);
        }
    }

    #[test]
    fn is_target_position_when_heigh_is_less_than_2() {
        let input = create_input(&vec![String::from(".#.")]);
        let actual = input.is_target_position(0, 0);
        assert_eq!(actual, false);
    }

    #[test]
    fn is_target_position_when_width_is_less_than_2() {
        let input = create_input(&vec![
            String::from("."),
            String::from("#"),
            String::from("."),
        ]);
        let actual = input.is_target_position(0, 0);
        assert_eq!(actual, false);
    }

    fn create_input(map: &Vec<String>) -> SnakeMapBossInput {
        SnakeMapBossInput {
            map: map.clone(),
            height: map.len(),
            width: map[0].len(),
        }
    }
}
