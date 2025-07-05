# Patterns and Matching
- Patterns are a special syntax in Rust for matching agianst the structure of types, both complex and simple, using patterns in conjuntion with `max` expressions and other constructs give more control over a program's control flow, a pattern consistens of some combination of the following:
    - Literals
    - Destructured arrays, enums, structs, or tuples
    - Varoables
    - Wildcards
    - Placeholders
- Some example patterns include, `x`, `(a, 3)`, and `Some::(Color::Red)`, in the contexts in which patterns are valid, these components describe the shape of data, program then matches values against the patterns to determine whether it has the correct shape of data to continue running a particular piece of code
- To use a pattern, it is compared to some value, if the pattern matches the value, use the value parts in code, if the value fits the shape of the pattern, can use the named pieces, if it doesn't, the code associated with the pattern won't run
- Will cover the valid places to use patterns, the difference between refutable and irrefutable patterns, and the different kinds of pattern syntax

## All the Places Patterns Can Be Used
### `match` Arms
- Patterns are used in the arms of `match` expressions, formally `match` expressions are defined as the keyword `match`, a value to match on, and one or more match arms that consist of a pattern and an expreszion to run if the value matches the arm's pattern like this:
- Example: ```
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}```
- Here is a `match` expression that matches on an `Option<i32>` value in the variable `x`:
```match x {
    Some(i) => Some(i + 1),
    None => None,
}```
- The patterns in the match exxpresion are the `None` and `Some(i)` to the left of each arrow
- One requirement for `match` expressions is that they need to be exhaustive in the sense that all possibilities for the value in the `match` expression must be accounted for, one way to ensure all patterns have been caught is to have a catchall pattern for the last arm, for example, a variable name matching any value can never fail and thus covers the remaining values
- The particular pattern `_` will match anything but it never binds to a variable so it's often used in the last `match` arm, the `_` pattern can be useful when ignoring any unspecified value

### Conditional `if let` Expressions
- `if let` expressions are a shorter way to write the equivalent of a `amatch` that only matches one case, `if let` can have a corresponding `else` containing code to run if the pattern in the `if let` doesn't match
- It's also possible to mix and match `if lt`, `else if`, and `else if let` expressions, doing so gives more flexibility than a match expression where can express only one value to compare with the patterns, also Rust doesn't require that the conditions of the `if let`, `else if`, and `else if let` arms relate to each other
- This code deteremines what color to make background based on a series of checks for several conditions
- Exmaple: ```
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("using favorite color {color}");
    } else if is_tuesday {
        println!("tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("purple");
        } else {
            println!("orange");
        }
    } else {
        println!("using blue");
    }
}```
- If the user specifies a favorite color, that color is used in the background, otherwise,  if the today is Tuesday, background is green, otherwise if age is specified as a string, can parse it as a number successfully, the color is either purple or orange, depending on the value of the number, if none of these conditions apply, the color is blue
- This conditional structure allows supporting complex requirements
- `if let` can also introduce new variables that shadow existing variables in the same way that `match arms` can, the line of `if let Ok(age) = age` introduces a new `age` variable that contains the value inside the `Ok` variant, shadowing the existing `age` variable, this means that the `if age > 30` condition needs to be placed inside that block, can't combine the two conditions into `if let Ok(age) = age && age > 30`, the new `age` isn't valid until the new scope starts with the curly braces
- The downside of using `if let` expressions is that the compiler doesn't check for exhaustiveness wherewas with `match` expressions it does, if missed the last `else` block and therefore missed handling some cases, compiler would not alert to possible logic bug

### `while let` Conditional Loops
- Similar to `if let`, the `while let` conditional loop allows a `while` loop to run for as long as a pattern cotinues to match, here is a `while let` loop that waits on messages sent between threads, in this case checks a `Result` instead of `Option`
- Example: ```
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("value");
    }```
- This example prints `1`, `2`, `3`, the `recv` method takes the first message out of the receiver side of the channel and returns an `Ok(value)`, earlier, unwrapped the error directly or interacted with it as an iterator using a `for` loop, can use `while let` because the `recv` method returns `Ok` each time a message arrives as long as the sender exists and then produces an `Err` once the sender side disconnects

### `for` Loops
- In a `for` loop, value that directly follows the keyword `for` is a pattern, for example, `for x in y`, the `x` is that pattern
- Example: ```
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at {index}");
    }```
- An iterator is adapted using the `enumerate` method so it produces a value and the index for that value and the index for that value, placed in a tuple
- When this value is matched to the pattern `(index, value)`, `index` will be `0` and `value` will be `a`, printing the first line of output

### `let` Statements
- Patterns are used in `let` statements, for example, with straightforward variable assignment: `let x = 5;`, everytime a `let` is used, patterns have been used, a `let` statement may look like `let PATTERN = EXPRESSION;`
- In statements like `let x = 5`, with a varibale name in the `PATTERN` slot, the variable  name is just a particularly simple form of a pattern, Rust compares the expression against the pattern and assigns any names it finds
    - In `let x = 5;` example, `x` is a pattern that means bind what matches here to the variable `x`, since the name `x` is the whole pattern, this pattern effectively means "bind everything to the variable `x` whatever the value is
    - Example: `let (x, y, z) = (1, 2, 3);` showcases the pattern-matching aspect of `let` which uses a pattern with `let` to destructure a tuple
    - A tuple is matched against a pattern, Rust compares the value `(1, 2, 3)` to the pattern `(x, y, z)` and sees that the value matches hte pattern, in that the number of elements is the same in both, so Rust binds `1` to `x`, `2` to `y`, adn `3` to `z`, can think of this tuple pattern as nesting three individual variable patterns inside it
    - If the number of elements in the pattern doesn't match the number of elements in thye tuple, the overall type won't match and will get compiler error
    - Example: `let (x, y) = (1, 2, 3);`
        - This will result in a compile-time type error
    - Could fix this by ignoring one or more of the values in the tuple using `_` or `..`, if the pattern has too many variables in the pattern, the solution is to make the types match by removing variables so the number of variables equals the number of elements in the tuple

### Function Parameters
- Function parameters can also be patterns
- Example: ```
fn foo(x: i32) {}```
- This declares a function named `foo` that takes one parameter naed `x` of type `i32`
- The `x` part is a pattern, as done with `let`, could match a tuple in the function's arguments to the pattern
- Example: ```
fn main() {
    let point = (1, 2);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("current location: ({x}, {y})");
}```
- This code prints `current location: (1, 2)`, the values `&(1, 2)` match the pattern `&(x, y)` so `x` is the value `1` and `y` is the value `2`
- Can also use patterns in closure parameter lists in the same way as in function parameter lists, because closures are similar to functions
- Patterns don't work the same in every place they can be used, in some places patterns must be irrefutable, in other circumstances, they can be refutable

## Refutability: Whether a Pattern Might Fail to Match
- Patterns come in two forms: refutable and irreftable, patterns that will match for any possible value are irrefutable, and exmaple would be `x` in the statement `let x = 5;`, since `x` matches anything and therefore cannot fail to match, patterns that can fail to match for some possible value are refutable, an example would be `Some(x)` in the expression `if let Some(x) = a_value` becaus eif the value in the `a_value` variable is `None` rather than `Some`, the `Some(x)` pattern will not match
- Function parameters, `let` statements, and `for` loops can only accept irrefutable patterns because the program cannot do anything meaningful when values don't match, the `if let` and `while let` expressions and the `let..else` statement accept irrefutable patterns but hte compiler wanrs against irrefutable patterns because, by definition, they're intended to handle possible failure, the functionality of a conditional is in its ability to perform differently depending on success or failure
- Shouldn't have to worry about the distinction between refutable and irrefutable patterns, do need familiarity with concept of refutability, will need to change either the pattern or the construct using the pattern with, depending on the intended behavior of the code
- What happens when using a refutable pattern where Rust requires an irrefutable pattern and vice versa, here is `let` statement but for the pattern, have specified `Some(x)`, a refutable pattern: `let Some(x) = some_option_value`, this won't compile
    - If `some_option_value` were a `None` value, it would fail to match the pattern `Some(x)`, meaning the pattern is refutable, however if the `let` statement can only accept an irrefutable pattern because there is nothing valid the code can do with a `None` value, at compile time, Rust will complain that a refutable pattern was tried to be used where an irrefutable pattern is required
    - Since every valid value with the pattern `Some(x)` wasn't covered, Rust produces a compiler error
    - If there is a refutable pattern where an irrefutable pattern is needed, can fix it by changing the code that uses the pattern: instead of using `let`, can use `if let`, then if the pattern doesn't match, the code will just skip the code in the curly brackets, giving it a way to continue validity
- Example: ```
let Some(x) = some_option_value else {
    return;
}```
- This code is perfectly valid now, if given `if let` an irrefutable pattern, a pattern that will always match, such as `x`, the compiler will give a warning
- Example: ```let x = 5 else {
    return
}```
- Rust comaplins that it doesn't make sense to use `if let` with an irrefutable pattern
- For this reason, match arms must use refutable patterns, except for the last arm, which should match any remaining values
- Rust allows use of an irrefutable pattern in a `match` with only one arm, but this syntax isn't useful and could be replaced with a `let` statement

## Pattern Syntax
- All the syntax that is valid in patterns and discuss why and when may want to use each one

### Matching Literals
- Can match patterns against literals directly
- Example: ```
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything"),
    }```
- This code prints `one` because the value in `x` is `1`, this syntax is useful when wanting the code to take an action if it gets a particular concrete value

### Matching Named Variables
- Named variables are irrefutable patterns that match any value, there is a complication when using named variables in `match`, `if let`, or `while let` expressions, since each of these kinds of expressions starts a new scope, variables declayred as part of a pattern inside the expression will shadow those with the name outside as is the case with all variables
- Have declared a variable named `x` with the value `Some(5)` and a variable named `y` with the value `10`, then create a `match` expression on the value `x`
- Example: ```
    let x = Some(1);
    let y = 10;

    match x {
        Some(50) => println!("got 50"),
        Some(y) => println!("{y}"),
        _ => println!("default case, x = {x:?}"),
    }

    println!("x = {x:?}, y = {y}");
}```
- Pattern in the first match arm doesn't match the defined value of `x` so the code continues
- The pattern in the second match arm introduces a new variable named `y`  that will match any value inside a `Some` value, since this is in a new scope inside the `match` expression, this is a new `y` variable, not the `y` declared at the beginnin gwith the value `10`, this new `y` binding will match any value inside a `Some` which is what there is in `x`, this new `y` binds to the inner value of the `Some` in `x`, that value is `5`, the expression for that arm executes and prints `1`
- If `x` had been a `None` value instead of `Some(5)`, the patterns in the first two arms wouldn't have matched so the value would have matched to the underscore, didn't introduce the `x` variable in the pattern of the underscore arm, so the `x` in the expression is stil the other `x` that hasn't been shadowed, in this case, the `match` would print `default case, x = None`
- When the `match` expression is done, its scope ends, and so does the scope of the inner `y`, the last `println!` prdocues `x = Some(1), y = 10`
- To create a `match` expression that compares the values of the outer `x` and `y` rather than introducing a new variable that shadows the existing `y` variable, would need to use a match guard conditional sintead

### Mulitple Patterns
- Can match multiple patterns using the `|` syntax, which is the pattern or operator, in following code, match the value of `x` against the match arms, the first of which has an or option, meaning if the value of `x` mathches either of the values in that arm, that arm's code will run
- Example: ```
    let x = 1;
    
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("{anything}"),
    }```
- This code prints `one or two`

### Matching Ranges of Values with `..=`
- The `..=` syntax allows matching an inclusive range of values, when a pattern matches any of the values within the given range, the arm will execute
- Example: ```
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }```
- If `x` is an integer `1` through `5`, the first arm will match, this syntax is more convenient for mulitple match values than using the `|` operator to express the same idea, to use the `|` would have to specify each distinct value `1` through `5`, specifying a range is much shorter, especially when intending to match any number in a larger range such as `1` though `1000`
- The compiler checks that the range isn't empty at compile time and because the only types for which Rust can tell if a range is empty or not are `char` and numeric values, ranges are only allowed with numeric or `char` values
- Example: ```
    let x = 'c';

    match x {
        'a'..='j' => println!("early ascii letter"),
        'k'..='z' => println!("late ascii letter"),
        _ println!("something else"),
    }```
- Rust can tell that `'c` is within the first pattern's range and prints `early ascii latter`

### Descructuring to Break Apart Values
- Can also use patterns to destructure structs, enums, and tuples to use different parts of these values

#### Destructuring Structs
- This shows a `Point` struct with two fields, `x` and `y` that can be broken apart using a pattern with a `let` statement
- Example: ```
fn main() {
    let p = Point { x: 0, y: 1 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(1, b);
}

struct Point {
    x: i32,
    y: i32,
}```
- This code creates the variables `a` and `b` that match the values of the `x` and `y` fields of the `p` struct, this example shows that the names of the variables in the pattern don't have to match the field name of the struct, however, it's common to match the variable names to the field names to make it easier to remember which variables come from which fields, because of this common usage and because writing `let Point { x: x, y: y } = p;` contains a lot of duplication, Rust has a shorthand for patterns that match struct fields, only need to list the name of the struct field and the variables created from the pattern and will have the same names, the following code behaves the same but the variables created in the `let` pattern are `x` ad `y` instead
- Example: ```
```
    let p = Point { x: 0, y: 1 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(1, y);```
- This code creates the variables `x` and `y` that match the `x` and `y` fields of the `p` variable, the outcome is that the variables `x` and `y` contain teh values from the `p` struct
- Can also destructure with literal values as part of the struct pattern rather than creating variables to destructure other fields
- Below, have a `match` expression that separates `Point` values into three cases: points that lie directly on the `x` axis (true when `y = 0`) and on the `y` axis, `(x = 0)` or on neither axis
- Example: ```
    match p {
        Point { x, y: 0 } => println!("on the x axis x = {x}"),
        Point { x: 0, y } => println!("on the y axis y = {y}"),
        Point { x, y } => {
            println!("on neither axis ({x}, {y})");
        }
    }```
- The first arm will match any point that lies on the `x` axis by specifying that the `y` field matches if its value matches the literal `0`, the pattern still creates an `x` variable that can be used in code for this arm
- Similarly, the second arm matches any point on the `y` axis by specifying that the `x` field matches if its value is `0` and creates a variable `y` for the value of the `y` field, the third arm doesn't specify any literals so it matches any other `Point` and creates variables for both the `x` and `y` fields
- In this example, the value `p` matches the second arm by virtue of `x` containing a `0`, so this code will print `on the y axis at y = 1`
- Remember that a `match` expression stops checking arms once it has found the first matching pattern, so even though `Point { x: 0, y: 0 }` is on the `x` axis and the `y` axis, this code would only print `on the x axis at x = 0`

### Destructuring Enums
- The pattern to explicitly destructure enums corresponds to the way the data stored within the enum is defined
- Example: ```
    let msg = Message::ChangeColor(0, 100, 255);

    match msg {
        Message::Quit => {
            println!("quit");
        },
        Message::ChangeColor(r, g, b) => {
            println!("{r}, {g}, {b}");
        },
        _ => println!("other"),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}```
- This code will print the RGB values of change color: `0, 100, 255`
- For enum variants without any data like `Message::Quit`, can't destructure the value any further, can only match the literal `Message::Quit` value, and no variables are in that pattern
- For struct-like enum variants such as `Message::Move`, can use a pattern similar to the pattern used to specify structs, after the variant name, can place curly brackets and then list the fields with variables to break apart the pieces to use in the code for this arm
- For tuple-like enum variants such as `Message::Write` that holds a tuple with one element and `Message::ChangeColor` that holds a tuple with three elements, the pattern is similar to the pattern specified to match tuples, the number of variables in the pattern must match the number of elements in the variant matched

#### Destructuring Nested Strucs and Enums
- Matching can work on nested items too
- Example: ```
    let msg = Message::ChangeColor(Color::Rgb(0, 100, 255));

    match msg {
        Message::Quit => {
            println!("quit");
        },
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("{r}, {g}, {b}");
        },
        _ => println!("other"),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}```
- The pattern of the first arm in the `match` expression matches a `Message::ChangeColor` enum variant that contains a `Color::Rgb` variant, then the pattern bends to the three inner `i32` values, can specify these complex conditions in one `match` expression despite multiple enums being involved

#### Destructuring Structs and Tuples
- Can mix and match and nest destructuring patterns in even more complex ways, the following example shows a destrucutre with nested structures and tuples inside a tuple and destructure all the primitive values out
- Example: `let ((feet, inches), Point { x, y }) =  ((3, 10), Point { x: 0, y: 1});`
- This code allows the decomposition of complex types into their component parts to use the values of interest separately
- Destructuring with patterns is a convenient way to use pieces of values such as the value from each field in a struct separately from each other

### Ignoring Values in a Pattern
- There are a few ways to ignore entire values or parts of values in a pattern using the `_` pattern, using a name that starts with an underscore, or using `..` to ignore the remaining parts of a value

#### An Entire Value with `_`
- Have used the `_` as a wildcard pattern that will match any value but not bind to it, this is useful as the last arm in a `match` expression, but can also use it in any pattern, including function parameters
- Example: ```
fn main() {
    foo(1, 2);
}

fn foo(_: i32, y: i32) {
    println!("this code uses the y param {y}");
}```
- This code will ignore the `1` value pased as the first argument and will print `this code uses the y param 2`
- In most cases, when no longer needing a particular function parameter, would change the signature so it doesn't include the unused parameter, ignoring a function parameter can be especially useful in cases when implementing a trait and need to fulfill a type signature but the function body in implementation doesn't need one of the parameters, then avoid getting a compiler warning about unused function parameters

#### Parts of a Value with a Nested `_`
- Can also use `_` inside another pattern to ignore just part of a value, for example, when wanting to test for only part of a value but have no use for the other parts in the corresponding code to run
- This requires that the user should not be allowed to overwrite an existing customization of a setting but can unset the setting and give it a value if it is currently unset
- Example: ```
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("cannot overwrite an existing customized value"),
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");```
- This code will print `cannot overwrite an existing customized value` and then `setting is Some(5)`
- In the first match arm, don't need to match on or use the values inside either `Some` variant but do need to test for the case when `setting_value` and `new_setting_value` are the `Some` variant, in this case, will print the reason for not changing `setting_value`, and it doesn't get changed
- In all other cases, allow `new_setting_value` to become `setting_value`
- Can also use underscores in multiple places within one pattern to ignore particular values
- Example: ```
    let numbers = (1, 2, 3);

    match numbers {
        (_, second, _) => println!("{second}"),
    }```

#### An Unused Variable by Starting Its Name with `_`
- If creating a variable but don't use it anywhere, Rust will issue warning because unused variable could be a bug, sometimes useful to create a variable that isn't in use yet, such as when prototyping or starting a project, in this situation can ctell Rust no tto warn about the unused variable by starting the name of the variable with an underscore
- Example: ```
    let _x = 10;
    let y = 10;```
- Will receive a warning about not using `y` but no warning about not using `x`
- Subtle difference between using only `_` and using a name that starts iwth an underscore, the syntax `_x` still binds the value to the variable, whereas `_` doesn't bind at all, to show a case where the distinction matters, there is the following code
- Example: ```
    let s = Some(String::from("hi"));

    if let Some(_s) = s {
        println!("found a string");
    }

    println!("{s:?}");```
- An unused variable starting with an underscore still binds the value which might take ownership of the value
- Receive an error here because the `s` value will still be moved into `_s` which prevents using `s` again
- The underscore by itself doesn't ever bind to the value
- Example: ```
    let s = Some(String::from("hi"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}");```
- This code compiles without errors `s` doesn't get moved into `_`
- The code works just fine because never bind `s` to anything and it isn't moved

#### Remaining Parts of a Value with `...`
- With values that have many parts, can use the `..` syntax to use specific parts and ignore the rest, avoiding to list many underscores for each ignored value, the `..` pattern ignores any parts of a value that haven't explicitly matched in the rest of the pattern
- Have a `Point` struct that holds a coordinate in 3D space, in the `match` expression want to operate only on the `x` coordinate and ignore the values in `y` and `z` fields
- Example: ```
    let origin = NewPoint { x: 0, y: 1, z: 2 };

    match origin {
        NewPoint { x, .. } => println!("x is {x}"),
    }```
- Listed the `x` value and then just include the `..` pattern, this is quicker than having to list `y: _` and `z: _`, when working with structs that have lots of fields in situations where only one or two fields are relevant
- The syntax `..` will expand to as many values as necessary, can use `..` with tuple
- Example: ```
fn main () {
    let numbers = (1, 2, 3);

    match numbers {
        (first, .., last) => println!("{first} {last}"),
    }
}```
- The first and last value are matched with `first` and `last`, the `..` will match and ignore everything in the middle
- Using `..` must be unambiguous, if it is unclear which values are intended for matching and which should be ignored, Rust will give an error, here is an example of using `..` ambiguously, here is an example of `..` being used ambiguously: ```
let numbers = (1, 2, 3);

match numbers {
    (.., second, ..) => {
        println!("{second}");
    }
}```
- It's impossible for Rust to determine how many values in the tuple to ignore before matching a value with `second` and then how many further values to ignore after, will get a compiler error because using `..` in two places like this is ambiguous

### Extra Conditionals with Match Guards
- A match guard is an additional if statement, specified after the pattern in a match arm that must also match for that arm to be chosen, match guards are useful for expressing more complex ideas than a pattern aline allows, however, they are only available in `match` expressions, not in `if let` or `while let` expressions
- The condition can use variables created in the pattern
- Example: ```
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("{x} is even"),
        Some(x) => println!("{x} is odd"),
        None => (),
    }```
- This will print `4 is even`, when `num` is compared to the pattern in the first arm, it matches because `Some(4)` matches `Some(x)`, then the match guard checks whether the remainder of dividing `x` by 2 is equal to 0 and because it is, the first arm is selected
- If `num` had been `Some(5)` instead, the match guard in the first arm would have been `false` because 5 % 2 == 1 which != 0, Rust would then go to the second arm which would match because the second arm doesn't have a match guard and therefore matches any `Some` variant
- No way to express `if x % 2 == 0` condition within a pattern so the match guard gives the ability to express this logic, the downside of this additional expressiveness is the compiler doesn't try to check for exhaustiveness when match guard expressions are involved
- Example: ```
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("50"),
        Some(n) if n == y => println!("matched n = {n}"),
        _ => println!("{x:?}"),
    }```
- This code will print `5`, the pattern in the second match arm doesn't introduce a new variable `y` that would shadow the outer `y`, meaning can use the outer `y` in the match guard instead of specifying the pattern as `Some(y)`, which would have shadowed the outer `y`, `Some(n)` is specified, creates a new variable `n` that doesn't shadow anything because there is no `n` variable outside the match
- The match guard `if n == y` is not a pattern and therefore doesn't introduce new variables, this `y` is the outer `y` rather than a new `y` shadowing it, can look for a value that has the same value as the outer `y` by comparing `n` to `y`
- Can also use the or operator `|` in a match guard to specify multiple patterns, the match guard conditional will apply to all the patterns
- Example: ```
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }```
- The match condition states that the arm only mathes if the value of `x` is `4`, `5`, or `6` and if `y` is true, when this code runs, the pattern of the first arm matches because `x` is `4`, but the match guard `if y` is `false`, so the first arm is not chosen, code moves on to second arm which does not match and program prints `no`, reason is that the `if` condition applies to the whole pattern `4 | 5 | 6`, not just the last value `6`, precendenace of a match guard in releation to a pattern behaves like this: `(4 | 5 | 6) if y => ...`, if this wasn't the case, program may have printed `yes`

### `@` Bindings
- The at operator `@` allows creation of a variable that holds a value at the same time that value is tested for a pattern match
- The want to test that a `Message::Hello` id is within the range `3..=7`, also want to bind the variable `id_variable` so can use it in the code associated with the arm, can name this variable `id`
- Example: ```
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("found an id in range: {id_variable}"),
        Message::Hello {
            id: 8..=12
        } => {
            println!("id in another");
        }
        _ => println!("other"),
    }```
- This example will print `Found an id in range: 5`, by specifying `id_variable @` before the range `3..=7`, this captures whatever value matched the range while also testing that the value matched the range pattern
- Second arm, only have a range specified in the pattern , the code associated with the arm doesn't have a variable that contains the actual value of the `id` field, pattern code is unable to use the value from the `id` field since the `id` has not been saved in a variable
- Using `@` allows testing and saving a value within one pattern

### Summary
- Rust's patterns are very useful in distinguishing between different kinds of data, when used in `match` expressions, Rust ensures patterns cover every possible value or program won't compile, patterns in `let` statements and function parameters make those constructs more useful, enabling the destructuring of values into smaller parts at the same time as assigning those parts to variables, can create simple or complex patterns so suit needs
