# Pest Tree
An alternative to `pest_ast` for converting dynamically typed pest trees into statically typed ones, using macros

### Goals
-   Intuitive
-   Explicit
-   Easy to learn for beginners
### Non-goals
-   Fast
-   Works for every case
-   customization support

This crate revolves around the trait `PestTree` which can be derived for Structs and Enums.

# Options
### Basic
| Name          | Parameters                                     | Purpose                                                          |
|---------------|------------------------------------------------|------------------------------------------------------------------|
| strategy(...) | A `strategy`, such as `Direct` or `Sequential` | Specify a matcher strategy for the struct/enum.                  |
| require(...)  | A `Requirement`, such as `Rule(...)`           | Define requirements not covered by the struct's rules/strategy.  |
| convert(...)  | A `Converter`, such as `auto`                  | Convert the `Pairs` into something like an `i32` in a struct     |
### Strategy
| Name         | Parameters                        | Purpose                                                          |
|--------------|-----------------------------------|------------------------------------------------------------------|
| `Direct`     | None                              | Match tokens like `in`, `let`, etc.                              |
| `Sequential` | None                              | Match a series of tokens, such as `let a = 3;`                   |
### Requirement
| Name          | Parameters                                                                                 | Purpose                                                                                                                                        |
|---------------|--------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------|
| `rule(...)`     | A `Rule` enum variant generated by `pest_derive`                                           | Confirm that the `Pairs` matches a rule. This requirement should be used in most if not all structs/enums                                      |
| `validate(...)` | A closure or a function that accepts a `Pairs` and returns `true` if the input is accepted | Match based on custom conditions.                                                                                                              |
| `or(...)`       | Two or more `Requirement`s                                                                 | Only 1 requirement in `or(...)` have to match for it to be accepted. For functionality like `and(...)`, just use multiple `require` attributes |
### Convert
| Name          | Parameters                                                                                | Purpose                                                                                                                                    |
|---------------|-------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------|
| `custom_p(...)` | A closure or a function that takes a `Pair` and outputs the type it should convert to.    | Convert tokens into custom types.                                                                                                          |
| `custom_s(...)` | A closure or a function that accepts a `&str` and returns `true` if the input is accepted | Convert tokens into custom types.                                                                                                          |
| `auto`          | None                                                                                      | Automatically convert the `pair` into one of the basic types (`pest_tree` will determine the type based on your struct member declaration) |

<br><br><br>
---

PestTree needs helper attributes to determine how the tree can be parsed for example:

#### Directly matching a string/token
```pest
direct_match_a = { "a" }
```
```rust
// it's important to note that the Pest parser would have already figured out that rule direct_match_a
// can only be applied when the string is "a". There is no need to check for it./
#[derive(PestTree)]
// strategy(Direct) tells PestTree to directly match an empty struct
#[pest_tree(strategy(Direct))]
// Specifies the requirements for the struct to match
#[pest_tree(require(rule(Rule::direct_match_a)))]
struct A {}

// Alternative: use this macro, which will expand to the lines above.
pest_tree_direct_struct!(A, Rule::direct_match_a);
```
#### Match multiple strings/tokens
```pest
// assuming that a, b, c are "Direct" rules that have already been defined
abc = { a ~ b ~ c }
```
```rust
#[derive(PestTree)]
#[pest_tree(strategy(Sequential))]
#[pest_tree(require(rule(Rule::abc)))]
struct ABC {
    // If no attributes are specified (e.g. A and B), PestTree will just sequentially parse them. 
    a: A,
    b: B,
    // Convert C's pair into a string
    #[pest_tree(converter(pair_to_str))]
    c: &str
}
```