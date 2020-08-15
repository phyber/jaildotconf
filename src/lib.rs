use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "jail.conf.pest"]
pub struct JailDotConf;

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pest::{
        consumes_to,
        parses_to,
    };
    use pretty_assertions::assert_eq;
    use std::fs;
    use std::path::{
        Path,
        PathBuf,
    };

    fn test_data_path() -> PathBuf {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/test-data",
        );

        Path::new(path).to_path_buf()
    }

    fn read_input(s: &str) -> String {
        let filepath = test_data_path().join(s);

        fs::read_to_string(filepath).unwrap()
    }


    #[test]
    fn test_c_comment() {
        let input = read_input("c_comment.txt");

        let comment = JailDotConf::parse(Rule::c_comment, &input)
            .unwrap()
            .next()
            .unwrap()
            .as_str();

        let expected = indoc!(r#"
           /*
            * C comment /* test */
            */"#
        ).to_string();

        assert_eq!(expected, comment);
    }

    #[test]
    fn test_cpp_comment() {
        let input = read_input("cpp_comment.txt");

        let comment = JailDotConf::parse(Rule::cpp_comment, &input)
            .unwrap()
            .next()
            .unwrap()
            .as_str();

        let expected = "// CPP comment test".to_string();

        assert_eq!(expected, comment);
    }

    #[test]
    fn test_shell_comment_z() {
        let input = read_input("shell_comment.txt");

        parses_to!{
            parser: JailDotConf,
            input:  &input,
            rule:   Rule::shell_comment,
            tokens: [
                shell_comment(0, 20)
            ]
        }
    }

    #[test]
    fn test_shell_comment() {
        let input = read_input("shell_comment.txt");

        let comment = JailDotConf::parse(Rule::shell_comment, &input)
            .unwrap()
            .next()
            .unwrap()
            .as_str();

        let expected = "# Shell comment test".to_string();

        assert_eq!(expected, comment);
    }

    #[test]
    fn test_param_with_value() {
        // (input, is_ok)
        let tests = vec![
            ("allow.mount = true;",         true),
            ("allow.mount = \"true\";",     true),
            ("allow.mount = 'true';",       true),
            ("\"allow.mount\" = true;",     true),
            ("\"allow.mount\" = \"true\";", true),
            ("\"allow.mount\" = 'true';",   true),
            // Lack of trailing semi colon
            ("allow.mount = true",         false),
            ("allow.mount = \"true\"",     false),
            ("allow.mount = 'true'",       false),
            ("\"allow.mount\" = true",     false),
            ("\"allow.mount\" = \"true\"", false),
            ("\"allow.mount\" = 'true'",   false),
        ];

        for test in tests {
            let (input, ok) = test;
            let parsed = JailDotConf::parse(Rule::param_with_value, &input);

            if ok {
                let parsed = parsed
                    .unwrap()
                    .next()
                    .unwrap()
                    .as_str();

                // Input should be the same as parsed.
                assert_eq!(input, parsed);
            }
            else {
                assert!(parsed.is_err());
            }
        }
    }
}
