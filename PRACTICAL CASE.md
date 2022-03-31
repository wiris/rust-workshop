# Practical Case

Here we show a (very) simplified version of the actual translator we have on production to allow our customers to translate from LaTeX to MathML. Each box represents a step that our engine must perform to translate a given imput:

```other
       -———————————————————————————————————————————————————————-   
LaTeX  | -—————————————-    -—————————————-    -—————————————- |  MathML
—————> | | PREPROCESS  | —> |  TRANSLATE  | —> | POSTPROCESS | |  —————>
       | -—————————————-    -—————————————-    -—————————————- |
       -———————————————————————————————————————————————————————-
```

Each one of those steps requires a particular algorithm to do its job, which are:

* PREPROCESS: Tokenize the LaTeX input into their language units
* TRANSLATE: Translate each node of a given parse tree and flatten the structure into a string.
* POSTPROCESS: Minimize the output MathML by collapsing autoclosed tags.

For this practical case you are welcome to propose a solution for one of the mentioned algorithms.

> **This is not a test!** Choose the one you feel like you will have most fun with :)

## Option A - LaTeX Tokenizer

Before starting the translation phase our input must be splitted into what we call _Tokens_. A _Token_ represents meaninfull substring of the input, and for the purposes of this activity we stick to this practical definition for a small subset of LaTeX:

1. `\frac`, `\sqrt`, `\pm` and `\text` commands are Tokens.
2. Any non blank chararacter (not `' '`) is a Token.
3. A blank character (`' '`) is a Token **only** when placed inside a `\text{ ... }` command.

For instance:

* `'\frac{ 1 + 1 }{ 2 }'` is tokenized as `'\frac'`, `'{'`, `'1'`, `'+'`, `'1'`, `'}'`, `'{'`, `'2'`, `'}'`.
* `'\text{ Hello !}'` is tokenized as `'\text'`, `{`, `' '`, `'H'`, `'e'`, `'l'`, `'l'`, `'o'`, `' '`, `'!'`, `'}'`.

Your task is to implement the function that given a latex input `&str`, tokenizes it above and returns a `Vec<String>` of Tokens. To do so, open `/src/latex-tokenizer.rs` and implement the `tokenize` function.

## Option B - MathML Postprocess

After the translation phase we obtain a MathML string. However, this string may contain tags without any content such as the `mrow` on `<math><mfrac><mrow></mrow><mi>2</mi></math>` (obtained after translating the LaTeX input `\frac{}{2}` for instance). Such empty tags can be represented on MathML by what's called _autoclosing tag_: any XML tag `<myTag></myTag>` can be represented more concisely by `<myTag/>`.

Your task here is to implement a postprocess on the MathML output that collapses such empty tags, so that for instance:

* `<math><mfrac><mrow></mrow><mi>2</mi></math>` becomes `<math><mfrac><mrow/><mi>2</mi></math>`.
* `<math display="inline"></math>` becomes `<math display="inline"/>`.

Your task here is to implement the function that given a MathML input `&str`, process it by replacing any empty tag by its autoclosing equivalent one and returns a `String`. To do so, open `/src/mathml-postprocess.rs` and implement the `expand_autoclosing_tags` function.
