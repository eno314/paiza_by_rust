pub fn main() {
    let my_introduction = Introduction::of_me();
    println!("{}", my_introduction.get_name());
    println!("{}", my_introduction.get_language());
    println!("{}", my_introduction.get_hitokoto());
}

struct Introduction {
    name: String,
    language: String,
    hitokoto: String,
}

impl Introduction {
    fn get_name(&self) -> String {
        format!("name: {}", self.name)
    }

    fn get_language(&self) -> String {
        format!("language: {}", self.language)
    }

    fn get_hitokoto(&self) -> String {
        format!("hitokoto: {}", self.hitokoto)
    }

    fn of_me() -> Introduction {
        Introduction {
            name: String::from("eno314"),
            language: String::from("rust"),
            hitokoto: String::from("I'd like to master rust!"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_introduction_get_name() {
        let introduction = Introduction::of_me();

        let actual = introduction.get_name();
        assert_eq!(actual, "name: eno314");
    }

    #[test]
    fn test_introduction_get_language() {
        let introduction = Introduction::of_me();

        let actual = introduction.get_language();
        assert_eq!(actual, "language: rust");
    }

    #[test]
    fn test_introduction_get_hitokoto() {
        let introduction = Introduction::of_me();

        let actual = introduction.get_hitokoto();
        assert_eq!(actual, "hitokoto: I'd like to master rust!");
    }
}
