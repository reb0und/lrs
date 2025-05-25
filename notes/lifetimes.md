# Validating References with Lifetimes
- Lifetimes are another kind of generic, rather than ensuring that a type has ideal behavior, lifetimes ensure that references are valid as long as needed
- Every reference in Rust has a lifetime, which is the scope for which that reference is valid, most of the type lifetimes are implicit and inferred just like most of the time, types are inferred
- Have to annotate lifetimes when the lifetimes of references could be related in a few different ways, Rust requires the annotation of types only when multiple types are possible, Rust requires the annotation of relationships using generic lifetime parameters to ensure the actual referencs used at runtime will be valid

### Preventing Dangling Referencs with Lifetimes
- Main aim of lifetimes is to prevent dangling references which cause a program to reference data other than the data it's intended to reference
- Error says that `x` does not live long enough, `x` goes out of scopewhen the inner scope ends, `r` is still valid for the outer scope since its scope is larger and therefore it lives longer
- Rust determines that code is invalid using the borrow checker

### The Borrow Checker
- Compiler has a borrow checker that compares scopes to determine whether all borrows are valid
- Program will reject something in a longer lifetime if it refers to something in a shorter lifetime
- `r` can reference `x` because Rust knows that the reference in `r` will always be valid while `x` is valid

### Generic Lifetimes in Functions
- Example: ```
      fn longest(x: &str, y: &str) -> &str {
          if x.len() > y.len() { x } else { y }
      }```
- Return type needs generic lifetime parameters on it because Rust can't tell whether the reference being returned refers to `x` or `y`, to fix this error, adding generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis

### Lifetime Annotation Syntax
- Lifetime annotations don't change how long any of the references live, rather they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes
- Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter
- The names of lifetime parameters must start with an apostrophe `'` and are usually all lowercase and very short like generic types, example is `'a`, lifetime parametere annotations are placed after the `&` of a reference using a space to separate the annotation from the reference's type
- Examples: `&i32` a reference, `&'a i32`, a reference with an explicit lifetime, `&'a mut i32` a mutable reference with an explicit lifetime
- One lifetime annotation by itself doesn't have meaning because annotations are meant to tell Rust how generic lifetime parameters relate to each other in the context of the `longest` function

### Lifetime Annotations in Function Signatures
- To use lifetime annotations in function signatures, need to declare the generic lifetime parameters inside angle brackets between the function name and the parameter list, just as done with generic type parameters
- Signature expresses the following constraint: the returned reference will be valid as long as both parameters are valid, this is the relationship between lifetimes of the parameters and the return value, can name the lifeitme `'a` and add it to each reference
- Example: ```
         fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
             if x.len() > y.len() { x } else { y }
         }```
- This function signature tlels Rust that for some lifetime `'a` the function takes two parameters, both of which are string slices that live at least as long as lifetime `'a`, the function signature also indicates to Rust that the string slice returned from the function will live at least as long as lifetime `'a`, in practice, it means that the lifetime of the reference returned by the `longest` function is the same as the smaller of the lifetimes of the values referred to by the function arguments
- This does not change any of the lifetimes of any values passed in or returned but specifies to the borrow checker that it should reject any values that don't adhere to these constraints, `longest` does not need to know exactly how long `x` and `y` will live, only that some scope can be substituted for `'a` that will satisfy this signature
- When annotating lifetimes in functions, the annotations go in the function signature, not in the function body, the lifetime annotations become part of the contract of the function, similar to types in the signature, having function signatures contain the lifetime contract, means the analysis the Rust compiler does can be simpler
- If there is a problem with a functions annotations or calling, compiler errors can point to the part of code and constraints more precisely
- When passing concrete references to `longest`, the concrete lifetime that is substituted for `'a` is part of the scope of `x` that overlaps with the scope of `y`, the generic lifetime `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`, since the retuned reference has also been annotated with `'a`, the returned reference will also be valid for the length of the smaller lifetimes of `x` and `y`

### Thinking in Terms of Lifetimes
- When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters, otherwise must refer to a value created in the function but that would be a dangling reference because the value goes out of scope at the end
- Best fix would be to return an owned data type rather than a reference so the calling function is then responsible for cleaning up the value
- Lifetime syntax is about connecting the lifetimes of various parameters and return the values of functions, once connected, Rust has enough information to allow memory-safe operations and disallow operations t would create dangling pointers or otherwise violate memory safety

### Lifetime Annotations in Struct Definitions
- Can define structs to hold references, in this case need to add lifetime annotation on every reference in the struct's definition
- Example: ```
         struct ImportantExcerpt<'a> {
             part: &'a: str,
         }```
- This means an instance of `ImportantExcerpt` can't outlive reference in `part` field
- `main` creates an instance of the struct that holds a reference to astring slice owned by `n`, doesn't go out of scope until after the struct goes out of scope, so reference in the `ImportantExcerpt` instance is valid

### Lifetime Elision
- Example: `fn first_word(s: &str) -> &str {` is `fn first_word<'a>(s: &'a str) -> &'a str {`, Rust team found that programmers were entering same lifetime annotations over and over and wrote it into the compiler so borrow checker could infer the lifetimes in these situations and wouldn't need explicit annotations
- The patterns programmed into Rust's analysis of references are called the lifetime elision rules, sets of particular cases that the compiler will consider and if code fits these cases, don't need to write the lifetimes explicitly
- Elision rules don't provide full inference, if there is still ambiguity about what lifetimes the references have after Rust applies the rules, compiler won't guess what the lifetime of the remaining references should be, instead of guessing, compiler will provide error that can resolve by adding lifetime annotations
- Lifetimes on function and method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes
- Three rules (first is for input and second and third are for output lifetimes) for compiler to figure out the lifetimes of the references when there aren't explicit annotations, if compiler gets to the end of the three rules and there are still references for which it can't figure out lifetimes, compiler will stop with an error, these apply to `fn` definitions and `impl` blocks
- First rule is that the compiler assigns a lifetime parameter to each parameter that's a reference, a function with one parameter gets one lifetime parameter, a function with two parameters gets two separate lifetime parameters, etc
- Second rule is that if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
- Third rule is that if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, since this is a method, the lifetime of `self` is assigned to all output lifetime parameters, third rule makes it much nicer to read and write because fewer symbols are necessary

### Lifetime Annotations in Method Definitions
- When implementing methods on structs with lifetimes, use the same syntax as that of generic type parameters, where declaring and useing hte lifetime parameters depends on whether they're related to the struct fields or the method parameters and return values
- Lifetime names for struct fields always need to be declared after the `impl` keyword and then used after the struct's name because those lifetimes are part of the struct's type
- In method signarures inside the `impl` block, references might be tied to the lifetime of references in the struct's fields or they might be independent
- Lifetime elision rules make it so that lifetime annotations aren't necessary in method signatures
- Example: ```
      impl<'a> ImportantExcerpt<'a> {
          fn level(&self) -> u8 {
              3
          }
      }```
- Lifetime parameter declared after `impl` and its use after the type name are required, but not required to annotate the lifetime of the reference to `self` because of the first elision rule
- Example: ```
    fn announce(&self, announcement: &str) -> &str {
        self.part
    }```
   - Example of third rule, since two input lifetimes, Rust applies the first lifetime elision rule and gives both parmeters their own lifetimes, since one of the parameters is `&self` the return type gets the lifetime of `&self` and all lifetimes have been accounted for

### The Static Lifetime
- One special lifetime is `'static` which denotes that the affected reference can live for the entire duration of the program, all string literals have the `'static` lifetime
- Example: `let s: &'static str = "xyz";`
- Text of this string is stroed directly in program's binary, which is always available, therefore the lifetime of all string literals is `'static`
- Before specifying `'static` as the lifetime for a reference, think about whether the reference given is actually living the entire lifetime of a program or not and whether it should, most of the time, error suggesting the `'static` lifetime results from attempting to create a dangling reference or a mismatch of the available lifetimes, in such cases, solution is to fix those problems, not to specify the `'static` lifeitme

### Generic Type Parameters, Trait Bounds and Lifetimes Together
- Example: ```
      use std::fmt::Display;

      fn longest_with_an_announcement<'a, T>(
          x: &'a str,
          y: &'a str,
          ann: T,
      ) -> &'a str 
      where
          T: Display,
      {
          println!("{ann}");
          if x.len() > y.len() { x } else { y }
      }```
- Extra parameter, `ann` of generic type `T` which can be filled in by any type that implements the `Display` trait as specified by the `where` clause

### Summary
- Generic type parameters let code be applied to different types
- Traits and trait bounds ensure that even though the types are generic, they'll have the behavior the code needs
- Can use lifetime annotations to ensure that flexible code won't have any dangling references, all of this analysis will happen at compile time, which does not affect runtime performance
