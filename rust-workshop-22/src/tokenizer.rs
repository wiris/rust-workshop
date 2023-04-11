const LATEX_COMMANDS : [&str; 4] = ["\\frac", "\\sqrt", "\\pm" , "\\text"];

pub fn tokenize(latex: &str) -> Vec<String> {

    // Mutable empty string where the result should be built on
    let mut result: Vec<String> = Vec::new();

    // Vector of characters of the input, to be iterated.
    let latex_chars = latex.chars().collect::<Vec<char>>();

    // Wether we are inside a text command or not.
    let mut inside_text = false;

    let mut i = 0;
    while i < latex_chars.len() {
        match latex_chars[i] {
            '\\' => {
                let mut j = i;
                // We advance until the end of the command, which is a '{' character.
                while j < latex_chars.len() && latex_chars[j] != '{' {
                    j += 1;
                }
                i = j;
                // We get the comand as a string, removing any empty spaces at the end
                let command = String::from_iter(&latex_chars[i..j]);
                let command = command.trim().to_string();

                // TODO: Append `command` to result? Update `inside_text`?
            },
            '}' => {
                // TODO: Closing bracket. Append it to `result`? Update `inside_text`?
            }
            ' ' => {
                // TODO: Blank space. Append it to `result`?
            },
            c => {
                // TODO: Any other character. Append it to `result`?
            }
        }
        i += 1;
    }

    return result;
}


#[cfg(test)]
mod test {
    use crate::tokenizer::tokenize;

    fn check(latex: &str, expected: &str) {
        pretty_assertions::assert_str_eq!(
            expected,
            tokenize(latex).join(" "),
            "Tokenization does not match (left: Expected / right: Computed): "
        );
    }

    #[test]
    fn tokenize_example1() {
        check("\\frac{ 1 + 1 }{ 2 }", "\\frac { 1 + 1 } { 2 }");
    }

    #[test]
    fn tokenize_example1_nospaces() {
        check("\\frac{1+1}{2}", "\\frac { 1 + 1 } { 2 }");
    }

    #[test]
    fn tokenize_example2() {
        check("\\text{ Hello !}", "\\text {   H e l l o   ! }");
    }

    #[test]
    fn tokenize_quadratic_formula() {
        check("x \\text{ equals } \\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a}", 
        "x \\text {   e q u a l s   } \\frac { - b \\pm \\sqrt { b ^ 2 - 4 a c } } { 2 a }");
    }
}