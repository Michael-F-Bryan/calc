# Parsing

The first step in creating our calculator is turning a stream of text provided
by the user into something more computer-friendly. This structure is usually
referred to as an [Abstract Syntax Tree] and is essentially just a tree where 
each leaf node is an "atom" (the smallest possible construct in a language, 
usually constants or identifiers). All non-leaf nodes then correspond to the 
compound constructs such as binary operators or function calls.

To make things easier we'll be using [lalrpop] to generate our parsing code and
construct the AST. If you've never heard of `lalrpop` I *highly recommend* you
check out [their guide].


[Abstract Syntax Tree]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
[lalrpop]: https://github.com/lalrpop/lalrpop
[their guide]: http://lalrpop.github.io/lalrpop/README.html
[bnf]: https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form