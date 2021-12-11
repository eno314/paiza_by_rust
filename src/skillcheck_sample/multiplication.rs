use crate::utils;

pub fn main() {
    utils::resolve(MultiplicationResolver {});
}
struct MultiplicationResolver;

impl utils::Resolver for MultiplicationResolver {
    type Input = (u32, u32);

    fn read_input(&self) -> Self::Input {
        let a: u32 = utils::InputReader::read_line();
        let b: u32 = utils::InputReader::read_line();
        (a, b)
    }

    fn create_output(&self, input: Self::Input) -> String {
        let (a, b) = input;
        (a * b).to_string()
    }
}

#[cfg(test)]
mod tests {

    use crate::utils::Resolver;

    use super::*;

    #[test]
    fn test_create_output_1() {
        let input: (u32, u32) = (2, 3);
        let resolver = MultiplicationResolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, "6");
    }

    #[test]
    fn test_create_output_2() {
        let input: (u32, u32) = (0, 99);
        let resolver = MultiplicationResolver {};

        let actual = resolver.create_output(input);

        assert_eq!(actual, "0");
    }
}
