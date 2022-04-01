# Practical Case

Here we show a (very) simplified version of the actual translator we have on production to allow our customers to translate from LaTeX to MathML. Each box represents a step that our engine must perform to translate a given imput:

```other
        ┌───────────────────────────────────────────────────────┐   
LaTeX   │ ┌─────────────┐    ┌─────────────┐    ┌─────────────┐ │  MathML
─────>  │ │ PREPROCESS  │ -> │  TRANSLATE  │ -> │ POSTPROCESS │ │  ─────>
        │ └─────────────┘    └─────────────┘    └─────────────┘ │
        └───────────────────────────────────────────────────────┘
```

Each one of those steps requires a particular algorithm to do its job. Briefly those algorithms are:

* **Preprocess**: Split the LaTeX input into Tokens (also called _Tokenization_)
* **Translate**: Flatten an intermediate tree-like structure into the resulting MathML string.
* **Postprocess**: Minimize the MathML output by collapsing empty XML tags.

For this practical case you are welcome to propose a solution for **one** of the mentioned algorithms.

> **This is not a test!** Choose the one you feel like you will have most fun with :)

## Option A - LaTeX Tokenizer

Before starting the translation phase our input must be splitted into what we call _Tokens_. A _Token_ represents a substring of the input with an identified meaning, and we won't cover here all the possible tokenizations for the LaTeX language. Instead, for the purposes of this activity, we define a Token according to the following 3 rules:

1. `\frac`, `\sqrt`, `\pm` and `\text` are Tokens.
2. Any non blank chararacter (not `' '`) is a Token.
3. A blank character (`' '`) is a Token **only** when placed inside a `\text{ ... }` command.

For instance, the following input

```bash
'\frac{ 1 + 1 }{ 2 }'  #   spaces must be ignored
```

is tokenized as

```bash
['\frac', '{', '1', '+', '1', '}', '{', '2', '}']   
```

For another example, consider this other input with spaces inside a `\text` command:

```bash
'\text{ Hello !}'      #   spaces must NOT be ignored
```

its tokenization is

```bash
['\text', '{', ' ', 'H', 'e', 'l', 'l', 'o', ' ', '!', '}']
```

Your task is to implement the function that given a LaTeX input returns a vector of tokens according to the rules above. To do so fill the `tokenize` function on `/src/tokenizer.rs` with your implementation.

___

## Option B - Tree Reducer

At some point of the translation process we have an _Abstract Syntax Tree_ of the input but where each node has already been visited and transformed accordingly to a "pseudo MathML" containing **placeholders**. The goal then is to flatten this tree into a string with the help of placeholders to indicate where each flattened subtree must be included.

It is easier to see what that does look like with an example. Consider the folowing tree,possibly obtained after parsing and transforming the LaTeX input `\frac{1}{2}`:

```bash
Node: '<math> $1 </math>'                                #  Equation
  │
  └── Node: '<mfrac> $1 $2 </mfrac>'                     #  Fraction
        │
        ├── Node: '<mn> $1 </mn>'                        #  Numerator
        │     │
        │     └─ Leaf: '1'
        │
        └── Node: '<mn> $1 </mn>'                        #  Denominator
              │
              └─ Leaf: '2'
```

The `$n` inside the node strings are the placeholders we mentioned above. Any given `$i` must be replaced by the recursively flattened `i`th child of the node.

Bottom-up the first substitution would result in:

```bash
Node: '<math> $1 </math>'              
  │
  └── Node: '<mfrac> $1 $2 </mfrac>'  
        │
        ├── Leaf: '<mn> 1 </mn>'                          #  replaced $1 by '1'  
        │
        └── Leaf: '<mn> 2 </mn>'                          #  replaced $1 by '2' 

```

and then

```bash
Node: '<math> $1 </math>'               
  │
  └── Leaf: '<mfrac> <mn> 1 </mn> <mn> 2 </mn> </mfrac>'  #  $1 became '<mn> 1 </mn>' 
                                                          #  and $2 became '<mn> 2 </mn>'
```

and finally

```bash
Leaf: '<math> <mfrac> <mn> 1 </mn> <mn> 2 </mn> </mfrac> </math>'
```

Lets see another example. Consider the following tree:

```bash
Node: '<math> $1 </math>'                            
  │
  └── Node: '<mi> $1 </mi> <mtext> $1 $2 </mtext>'         #  $1 appears 2 times, and $3 none!   
        │
        ├── Leaf: 'π'                    
        │
        ├── Leaf: 'is also called the Circle constant!'
        │
        └── Leaf: 'i will be ignored :('
```

its reduction should result in the following string:

```bash
'<math> <mi> π </mi> <mtext> π is also called the Circle constant! </mtext> </math>'
```

Notice that:

* A subtree can appear multiple times if its placeholder is repeated.
* A subtree does not appear if its corresponding placeholder doesn't appear on the node above.

The purpose of this activity is to implement this flattening process. To do so implement the function `reduce_tree` on `/src/reducer.rs`. You'll also find the data definition of the tree representing this structure, which **you don't have to modify**.

___

## Option C - MathML Postprocess

After the translation phase we obtain a MathML string. However, this string may contain tags without any content such as the `mrow` on `<math><mfrac><mrow></mrow><mi>2</mi></math>`,obtained after translating the LaTeX input `\frac{}{2}` for instance. Such empty tags can be represented on MathML by what's called _autoclosing tag_: any XML tag of the form `<myTag></myTag>` can be represented more concisely by `<myTag/>`.

Your task here is to implement a postprocess on the MathML that collapses such empty tags, so that for instance:

```xml
<math><mfrac><mrow></mrow><mi>2</mi></math>
```

becomes

```xml
<math><mfrac><mrow/><mi>2</mi></math>        <!-- notice here the <mrow/> construct -->
```

Or if we have the following empty equation:

```xml
<math display="inline"></math>
```

the result of the postprocess is

```xml
<math display="inline"/>
```

More precisely, implement the function that given a MathML string, replaces any empty tag by its autoclosing equivalent. To do so, open `/src/mathml-postprocess.rs` and implement the `expand_autoclosing_tags` function.
___
