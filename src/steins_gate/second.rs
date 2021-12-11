use crate::utils;

pub fn main() {
    utils::resolve(SecondResolver {});
}

struct SecondResolver;

impl utils::Resolver for SecondResolver {
    type Input = SecondInput;

    fn read_input(&self) -> Self::Input {
        let first: Vec<f32> = utils::InputReader::read_line_splitted_by_whitespace();
        let size = first[0] as usize;
        SecondInput {
            exp_of_fight: first[1],
            exps_of_chars: utils::InputReader::read_lines(size),
        }
    }

    fn create_output(&self, input: Self::Input) -> String {
        let sum_of_exps_of_chars: f32 = input.exps_of_chars.iter().sum();
        let count = (sum_of_exps_of_chars / input.exp_of_fight).ceil();
        count.to_string()
    }
}

struct SecondInput {
    exp_of_fight: f32,
    exps_of_chars: Vec<f32>,
}

#[cfg(test)]
mod tests {

    use crate::utils::Resolver;

    use super::*;

    #[test]
    fn test_create_output_1() {
        let input = SecondInput {
            exp_of_fight: 2.5,
            exps_of_chars: vec![3.2, 1.2, 5.7],
        };
        let resolver = SecondResolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, "5");
    }

    #[test]
    fn test_create_output_2() {
        let input = SecondInput {
            exp_of_fight: 5.0,
            exps_of_chars: vec![5.0, 5.0, 7.5, 2.5],
        };
        let resolver = SecondResolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, "4");
    }
}
