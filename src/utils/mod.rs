pub fn resolve<T: Resolver>(resolver: T) {
    let input = resolver.read_input();
    let output = resolver.create_output(input);
    println!("{}", output);
}

pub trait Resolver {
    type Input;
    fn read_input(&self) -> Self::Input;
    fn create_output(&self, input: Self::Input) -> String;
}

pub struct InputReader;

impl InputReader {
    pub fn read_lines<T: std::str::FromStr>(times: usize) -> Vec<T> {
        (0..times).map(|_| InputReader::read_line()).collect()
    }

    pub fn read_lines_splitted_by_whitespace<T: std::str::FromStr>(times: usize) -> Vec<Vec<T>> {
        (0..times)
            .map(|_| InputReader::read_line_splitted_by_whitespace())
            .collect()
    }

    pub fn read_line_splitted_by_whitespace<T: std::str::FromStr>() -> Vec<T> {
        let line: String = InputReader::read_line();
        line.split_whitespace()
            .map(|c| c.parse().ok().unwrap())
            .collect()
    }

    pub fn read_line<T: std::str::FromStr>() -> T {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().ok().unwrap()
    }
}
