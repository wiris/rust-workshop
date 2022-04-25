const LATEX_COMMANDS : [&str; 4] = ["\\frac", "\\sqrt", "\\pm" , "\\text"];

pub fn tokenize(latex: &str) -> Vec<String> {

    // Mutable empty string where the result should be built on
    let mut result: Vec<String> = Vec::new();

    let mut inside_text = false;

    // For each char on the imput
    for c in latex.chars() {
        match c {
            '\\' => {
                // Command prefix. Read command? Append it to `result`? Update `inside_text`?
            },
            '}' => {
                // Closing bracket. Apend it to `result`? Update `inside_text`?
            }
            ' ' => { 
                // Blank soace. Apend it to `result`?
            },
            _ => {
                // Any other character. Apend it to `result`?
            }
        }
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
        check("\\frac{1+1}{2}", "\\frac{ 1 + 1 } { 2 }");
    }

    #[test]
    fn tokenize_example2() {
        check("\\text{ Hello !}", "\\text {   Hello ! } ");
    }

    #[test]
    fn tokenize_quadratic_formula() {
        check("x \\text{ equals } \\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a}", 
        "x \\text {   e q u a l s   } \\frac { - b \\pm \\sqrt { b ^ 2 - 4 a c } } { 2 a }");
    }
}