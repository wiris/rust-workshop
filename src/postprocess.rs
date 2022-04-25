
pub fn autoclose_tags(mathml: &str) -> String {

    // Mutable empty string where the result should be built on
    let mut result = String::new();

    // For each char on the imput
    for c in mathml.chars() {
        // TODO: do something. You are on your own here!
    }
    

    return result;
}

#[cfg(test)]
mod test {
    use crate::postprocess::autoclose_tags;

    fn check(mathml: &str, expected: &str) {
        pretty_assertions::assert_str_eq!(
            expected,
            autoclose_tags(mathml),
            "Postprocess does not match (left: Expected / right: Computed): "
        );
    }

    #[test]
    fn postprocess_simple() {
        check("<myTag></myTag>", "<myTag/>");
    }

    #[test]
    fn postprocess_empty_denominator() {
        check("<math><mfrac><mrow><mn>1</mn></mrow><mrow></mrow></math>", "<math><mfrac><mrow><mn>1</mn></mrow><mrow/></math>");
    }

    #[test]
    fn postprocess_empty_table() {
        check("<math><mtable><mtr><mtd></mtd><mtd></mtd></mtr><mtr><mtd></mtd><mtd></mtd></mtr></mtable></math>", "<math><mtable><mtr><mtd/><mtd/></mtr><mtr><mtd/><mtd/></mtr></mtable></math>");
    }


    #[test]
    fn postprocess_quadratic_formula() {
        check("<math><mfrac><mrow><mo>-</mo><mi>b</mi><mo>&#xB1;</mo><msqrt><msup><mi>b</mi><mn>2</mn></msup><mo>-</mo><mn>4</mn><mi>a</mi><mi>c</mi></msqrt></mrow><mrow><mn>2</mn><mi>a</mi></mrow></mfrac></math>", 
        "<math><mfrac><mrow><mo>-</mo><mi>b</mi><mo>&#xB1;</mo><msqrt><msup><mi>b</mi><mn>2</mn></msup><mo>-</mo><mn>4</mn><mi>a</mi><mi>c</mi></msqrt></mrow><mrow><mn>2</mn><mi>a</mi></mrow></mfrac></math>");
    }
}