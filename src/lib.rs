use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "jail.conf.pest"]
pub struct JailDotConf;

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
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
        let input = "allow.mount = true;";

        let parsed = JailDotConf::parse(Rule::param_with_value, &input)
            .unwrap()
            .next()
            .unwrap()
            .as_str();

        let expected = "allow.mount = true;";

        assert_eq!(expected, parsed);
    }
}
