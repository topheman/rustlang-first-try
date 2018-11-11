//! # Greeter implentations
//!
//! Greeter, implemented with both:
//! * builder pattern
//! * default trait
pub mod builder {
    //! Implementation using [builder pattern](https://github.com/rust-unofficial/patterns/blob/master/patterns/builder.md).
    //!
    //! That way, we provide defaults (rust doesn't have default values for params)
    //!
    //! [Check this video](https://youtu.be/STWuPMcwwbw) for more infos.
    use libs::language::Language;
    use std::fmt;

    #[derive(Debug)]
    pub struct Greeter {
        language: Language,
        name: String,
    }

    impl fmt::Display for Greeter {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let greeting: &str = match self.language {
                Language::English => "Hello",
                Language::French => "Bonjour",
            };
            write!(f, "{} {}", greeting, self.name)
        }
    }

    pub struct GreeterBuilder {
        language: Language,
        name: String,
    }

    impl GreeterBuilder {
        pub fn new() -> GreeterBuilder {
            GreeterBuilder {
                language: Language::English,
                name: String::new(),
            }
        }
        pub fn name(mut self, name: String) -> GreeterBuilder {
            self.name = name;
            self
        }
        pub fn with_language(mut self, language: Language) -> GreeterBuilder {
            self.language = language;
            self
        }
        /**
         * Also called `build` sometimes
         */
        pub fn finish(self) -> Greeter {
            Greeter {
                name: self.name,
                language: self.language,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        // Note this useful idiom: importing names from outer (for mod tests) scope.
        use super::*;

        #[test]
        fn greeter_builder_it_works_with_default_language() {
            let greeter = GreeterBuilder::new().name("Yolo".to_string()).finish();
            assert_eq!(format!("{}", greeter), "Hello Yolo");
        }
        #[test]
        fn greeter_builder_it_works_with_specific_language() {
            let greeter = GreeterBuilder::new()
                .name("Yolo".to_string())
                .with_language(Language::French)
                .finish();
            assert_eq!(format!("{}", greeter), "Bonjour Yolo");
        }
        #[test]
        fn greeter_builder_it_works_with_debug_output() {
            let greeter = GreeterBuilder::new().name("Yolo".to_string()).finish();
            assert_eq!(
                format!("{:?}", greeter),
                "Greeter { language: English, name: \"Yolo\" }"
            );
        }
    }
}

pub mod default {

    //! Implementation using [default trait](https://stackoverflow.com/questions/24047686/default-function-arguments-in-rust#31381497)
    use libs::language::Language;
    use std::default::Default;
    use std::fmt;

    #[derive(Debug)]
    pub struct Greeter {
        pub language: Language,
        pub name: String,
    }

    impl fmt::Display for Greeter {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let greeting: &str = match self.language {
                Language::English => "Hello",
                Language::French => "Bonjour",
            };
            write!(f, "{} {}", greeting, self.name)
        }
    }

    impl Default for Greeter {
        fn default() -> Self {
            Greeter {
                language: Language::English,
                name: "".to_string(),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        // only use what we need (not super::*)
        use super::Greeter;
        use super::Language;

        #[test]
        fn greeter_default_it_works_with_default_language() {
            let greeter = Greeter {
                name: "Yolo".to_string(),
                ..Greeter::default()
            };
            assert_eq!(format!("{}", greeter), "Hello Yolo");
        }
        #[test]
        fn greeter_default_it_works_with_specific_language() {
            let greeter = Greeter {
                name: "Yolo".to_string(),
                language: Language::French,
            };
            assert_eq!(format!("{}", greeter), "Bonjour Yolo");
        }
        #[test]
        fn greeter_default_it_works_with_debug_output() {
            let greeter = Greeter {
                name: "Yolo".to_string(),
                ..Greeter::default()
            };
            assert_eq!(
                format!("{:?}", greeter),
                "Greeter { language: English, name: \"Yolo\" }"
            );
        }
    }

}
