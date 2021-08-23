# toktok

toktok is a parser combinators library operating on tokens. While it is possible to write combinators by hand, toktok is best used with toktok-generator and a `.toktok` file like this:

```
...

value -> Value:
      map { Value::Object($1) }
    | array { Value::Array($1) }
    | string { Value::String($1) }
    | number { Value::Number($1) }
    | "true" { Value::Bool(true) }
    | "false" { Value::Bool(false) };

...
```

toktok-generator will then generate the combinators for you.

## Tokens

Creating tokens is not part of toktok. Bring your own tokens (e.g. [logos](https://crates.io/crates/logos)).

## Inspirations

- [lalrpop](https://github.com/lalrpop/lalrpop)
- [lrpar](https://github.com/softdevteam/grmtools)
- [pest](https://github.com/pest-parser/pest)
- [nom](https://github.com/Geal/nom)
