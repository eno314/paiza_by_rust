pub fn resolve<T: Resolver>(resolver: T) {
    let input = resolver.read_input();
    let output = resolver.create_output(input);
    println!("{}", output);
}

pub trait Resolver {
    type Input;
    fn read_input(&self) -> Self::Input;
    fn create_output(&self, input: Self::Input) -> String;

    fn read_lines<S: std::str::FromStr>(&self, times: usize) -> Vec<S> {
        (0..times).map(|_| self.read_line()).collect()
    }

    fn read_lines_splitted_by_whitespace<S: std::str::FromStr>(&self, times: usize) -> Vec<Vec<S>> {
        (0..times)
            .map(|_| self.read_line_splitted_by_whitespace())
            .collect()
    }

    fn read_line_splitted_by_whitespace<S: std::str::FromStr>(&self) -> Vec<S> {
        let line: String = self.read_line();
        line.split_whitespace()
            .map(|c| c.parse().ok().unwrap())
            .collect()
    }

    fn read_line<S: std::str::FromStr>(&self) -> S {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().ok().unwrap()
    }
}
