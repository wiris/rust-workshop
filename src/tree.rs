use crate::tree;

/// Reduce the tree into a String, substituting any pattern $i appearing on the node's 
/// with the recursive reduction of the i-th child.
pub fn reduce(tree: Tree) -> String {
    let childs = *tree.childs; // Owned vector of node's childs.
    let mut res = tree.node_str; // Owned mutable string of the node's content.

    
    for (i, child) in childs.into_iter().enumerate() {
        // replace $i on res with the recursive call to child!
    }

    return res;
}


pub fn example_tree() -> Tree {
    // You can build custom trees to test your function using the tree macro as follows:
    // tree![node_str; list of subtrees]
    tree!["<math> $1 </math>";
        tree!["<mfrac> $1 $2 </mfrac>";
            tree!["<mn> $1 </mn>";
                tree!["1"]
            ],
            tree!["<mn> $1 </mn>";
                tree!["2"]
            ]
        ]
    ]
}



/// Recursive Data Structure to represent the intermediate AST for this problem. DO NOT TOUCH!
pub struct Tree {
    node_str: String,       // String content of the node
    childs: Box<Vec<Tree>>, // List of subtrees under this node. Wrapped inside a `Box` reference to make the object Sized.
}

/// Macro to instantiate Trees more easily. DO NOT TOUCH!
#[macro_export]
macro_rules! tree {
    ($s:expr $(;$($childs:expr),*)?) => (crate::tree::Tree { node_str: $s.to_string(), childs: Box::new(vec![ $($($childs),*)? ]) })
}


#[cfg(test)]
mod test {
    use crate::tree;
    fn check(tree: tree::Tree, expected: &str) {
        pretty_assertions::assert_str_eq!(
            expected,
            tree::reduce(tree),
            "Recution does not match (left: Expected / right: Computed): "
        );
    }

    #[test]
    fn tree_example1() {
        check(
            tree!["<math> $1 </math>";
                tree!["<mfrac> $1 $2 </mfrac>";
                    tree!["<mn> $1 </mn>";
                        tree!["1"]
                    ],
                    tree!["<mn> $1 </mn>";
                        tree!["2"]
                    ]
                ]
            ],
            "<math> <mfrac> <mn> 1 </mn> <mn> 2 </mn> </mfrac> </math>",
        );
    }

    #[test]
    fn tree_example2() {
        check(
            tree!["<math> $1 </math>";
                tree!["<mi> $1 </mi> <mtext> $1 $2 </mtext>";
                    tree!["π"],
                    tree!["is also called the Circle constant!"],
                    tree!["i will be ignored :("]
                ]
            ],
            "<math> <mi> π </mi> <mtext> π is also called the Circle constant! </mtext> </math>",
        );
    }
}
