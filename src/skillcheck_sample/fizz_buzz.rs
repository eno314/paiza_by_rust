use crate::utils;

pub fn main() {
    utils::resolve(FizzBuzzResolver {});
}

struct FizzBuzzResolver;

impl utils::Resolver for FizzBuzzResolver {
    type Input = usize;

    fn read_input(&self) -> Self::Input {
        self.read_line()
    }

    fn create_output(&self, input: Self::Input) -> String {
        let fizz_buzz_values: Vec<String> = (1..=input).map(|n| fizz_buzz(n)).collect();
        fizz_buzz_values.join("\n")
    }
}

fn fizz_buzz(n: usize) -> String {
    if n % 15 == 0 {
        String::from("Fizz Buzz")
    } else if n % 5 == 0 {
        String::from("Buzz")
    } else if n % 3 == 0 {
        String::from("Fizz")
    } else {
        n.to_string()
    }
}

#[cfg(test)]
mod tests {

    use crate::utils::Resolver;

    use super::*;

    #[test]
    fn test_create_output() {
        let input: usize = 15;
        let resolver = FizzBuzzResolver {};

        let actual = resolver.create_output(input);

        let expected = vec![
            "1",
            "2",
            "Fizz",
            "4",
            "Buzz",
            "Fizz",
            "7",
            "8",
            "Fizz",
            "Buzz",
            "11",
            "Fizz",
            "13",
            "14",
            "Fizz Buzz",
        ]
        .join("\n");
        assert_eq!(actual, expected);
    }
}
