#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_macros)]

use cheat_sheet::showcase;
use ferris_says;
use std::io;
use tree::Tree;

mod tree;
mod tokenizer;
mod postprocess;
mod cheat_sheet;

fn main() {

    greet();

    tokenizer_example();
    tree_example();
    postprocess_example();

    showcase();
}

// Displays a greet from ferris 
fn greet() {
    let stdout = io::stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = io::BufWriter::new(stdout.lock());
    ferris_says::say(&message, width, &mut writer).unwrap();
    println!();
}


// Gets a tree and displays its reduction
fn tree_example() {
    // Load an example tree
    let tree: Tree = tree::example_tree(); 

    // Reduce it
    let reduction = tree::reduce(tree);

    // Display its reduction
    println!("TREE EXAMPLE\n-The result of the reduction is: {} \n", reduction);
}

fn tokenizer_example() {
    // Provided latex example
    let latex = "\\frac{1+1}{2}";

    // Compute the vector of tokens
    let tokens = tokenizer::tokenize(latex);

    println!("TOKENIZER EXAMPLE\n-The latex tokens for the input \"{}\" are: ", latex); 
    for token in tokens {
        // Display each token
        println!(" {}", token);
    }
    println!();
}


fn postprocess_example() {

    // Mahtml of the fraction 1/_
    let mathml = "<math><mfrac><mrow><mn>1</mn></mrow><mrow></mrow></math>";

    // Mathml equivalent using an autoclosing tag for the <mrow> on the denominator.
    let mathml_post = postprocess::autoclose_tags(mathml);
    
    // Display it
    println!("POSTPROCESS EXAMPLE\n-The result of applying a postprocess to this mathml: {} \n  is the following one: {}\n", mathml, mathml_post);
}