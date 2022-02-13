use crate::utils;

pub fn main() {
    utils::resolve(FirstResolver {});
}

struct FirstResolver;

impl utils::Resolver for FirstResolver {
    type Input = (String, String);

    fn read_input(&self) -> Self::Input {
        let first = self.read_line();
        let second = self.read_line();
        (first, second)
    }

    fn create_output(&self, input: Self::Input) -> String {
        let (first, second) = input;
        format!("{};{}", first, second)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::Resolver;

    #[test]
    fn first() {
        let first_input = String::from("steins");
        let second_input = String::from("gate");
        let resolver = FirstResolver {};

        let actual = resolver.create_output((first_input, second_input));

        assert_eq!(actual, String::from("steins;gate"))
    }
}
