# The Language Grammar

Now we've got a lot of the boilerplate set up, we can start trying to figure out
what our language's grammar should look like. 

The easiest way to do this is by writing out a bunch of example use cases.

```text
# This is a comment
5 * (3+4)  # You can do the usual arithmetic stuff
x = 3*PI/4  # and read/write variables
y = sin(x)^2 # plus call functions
```

While this language won't be turing complete (we don't have conditionals or 
loops), it should be a fairly decent calculator.

Once you have several examples the next step is to formalize the language 
grammar to make it easier to parse. This is usually done by writing a bunch of
"rules" in [Backus-Naur Form][bnf].

```ebnf
expr := <term>
      | "(" <expr> ")"
      | <function-call>
term := <factor>
      | <term> "+" <term>
      | <term> "-" <term>
factor := NUMBER
        | IDENTIFIER
        | <factor> "*" <factor>
        | <factor> "/" <factor>
function-call := IDENTIFIER "(" <arg-list> ")"
arg-list := EPSILON
          | <expr> ("," <expr>)*
```

To put it in human terms, we would read the first rule as saying "an *expr*
is either a *term*, an *expr* surrounded by parentheses, or a *function
call*".

