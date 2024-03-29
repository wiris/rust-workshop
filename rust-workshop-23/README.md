# Rust workshop 2023

[![wiris](https://custom-icon-badges.demolab.com/badge/Powered_by_the_WIRIS_team-red.svg?logo=heart&logoColor=white)](https://www.wiris.com/es/) [![rust](https://img.shields.io/badge/Rust-v1.68.0-orange.svg)](https://www.rust-lang.org/tools/install)

Welcome to the Rust Workshop 2023! Before starting, make sure you have read the [setup instructions](../README.md).

## About

At WIRIS we want to build amazing software that lets students understand and manipulate its elementary equations. However, we are having struggles implementing the __String to Object__ conversion.

A tree representation for the equation `"1+2*3-4/5"` would be

```mermaid
graph TD
A([sum]) --> t1((1))
A --> B([sub])

B --> C([prod])
B --> D([div])

C --> t2((2))
C --> t3((3))

D --> t4((4))
D --> t5((5))
```

We already know that the main structures that are required for achieving our objective are:

- The `Token`, which is an [enumeration](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) for representing each relevant particle of an expression
- And the `Node`, which is the smallest unit of the [binary tree](https://en.wikipedia.org/wiki/Binary_tree) we want to use for representing an expression in memory.  

We also agree that, at least, we need two different methods:

- The `tokenizer`, which is in charge of converting a string into an array of `Tokens`.
- And the `deserializer`, which should take advantage of the `tokenizer` for converting a string into a binary tree.

## Objective

The objective of this workshop is to provide an implementation for both, the `tokenizer` and `deserializer` methods, that makes all the tests pass successfully.

Notice that only elementary operations are considered in the expected solution. Any other operations, decimal or negative numbers, are not allowed. Let's keep it simple!

Also, feel free to add any other structure or method you consider appropriate for the implementation. The only requirement is to keep the code we provide as it is, the rest is up to you.

You will find the source code [right here](./src/lib.rs).

Good luck, and enjoy! 😁

## Testing

In order to run all the tests and see what is up, open a terminal in this workshop's directory and run the following command:

``` bash
cargo test
```
