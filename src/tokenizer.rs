const LATEX_COMMANDS : [&str; 4] = ["\\frac", "\\sqrt", "\\pm" , "\\text"];
const STARTING_COMMAND_CHAR: char = '\\';
const STARTING_COMMAND_BODY_CHAR: char = '{';
const ENDING_COMMAND_BODY_CHAR: char = '}';
const BLANK_CHAR: char = ' ';

pub fn tokenize(latex: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let latex_chars = latex.chars().collect::<Vec<char>>();

    tokenize_vector(&latex_chars, &mut result, 0, latex_chars.len());
    
    return result;
}

fn tokenize_vector(chars: &[char], tokens: &mut Vec<String>, start: usize, end: usize) -> usize {
    if start >= end || chars.len() <= start {
        return 0;
    }

    let mut padding = start;
    let mut is_text = false;

    if let Some(cmd) = find_command(chars, &LATEX_COMMANDS, start) {
        is_text = cmd == LATEX_COMMANDS[3];

        padding += cmd.len();
        tokens.push(cmd);
    }
    
    let current = chars[padding];
    if current != BLANK_CHAR {
        tokens.push(current.to_string());
    }
    
    padding += 1;
    if current == STARTING_COMMAND_BODY_CHAR {
        padding += tokenize_command_body(chars, tokens, padding, end, is_text);
    }

    padding + tokenize_vector(chars, tokens, padding, end)
}

fn find_command(chars: &[char], commands: &[&str], index: usize) -> Option<String> {
    if chars.len() <= index || chars[index] != STARTING_COMMAND_CHAR {
        return None;
    }

    let cmd = get_command(chars, commands, index, index+1);
    if cmd.len() == 0 {
        return None;
    }

    if commands.iter().any(|latex_cmd| latex_cmd == &cmd) {
        return Some(cmd);
    }

    None
}

fn get_command(chars: &[char], commands: &[&str], begin: usize, end: usize) -> String {
    if begin >= end || chars.len() <= begin || chars.len() <= end {
        return "".to_string();
    }

    if chars[end] == STARTING_COMMAND_BODY_CHAR || chars[end] == BLANK_CHAR {
        return chars[begin..end].to_vec().into_iter().collect();
    }

    get_command(chars, commands, begin, end+1)
}

fn tokenize_command_body(chars: &[char], tokens: &mut Vec<String>, start: usize, end: usize, allow_blank: bool) -> usize {
    if start >= end || chars.len() <= start {
        return 0;
    }

    if let Some(_) = find_command(chars, &LATEX_COMMANDS, start) {
        return tokenize_vector(chars, tokens, start, end);
    }

    let current_char: char = chars[start];
    if current_char != BLANK_CHAR || allow_blank {
        tokens.push(current_char.to_string());
    }

    if current_char == ENDING_COMMAND_BODY_CHAR {
        return 1;
    }

    1 + tokenize_command_body(chars, tokens, start+1, end, allow_blank)
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