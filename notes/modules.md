# Manging Growing Projects with Packages, Crates, and Modules
- Can grou prelated functionality and separate code with distinct features to clarify where to find code that implements a particular feature and where to go to change how a feature works
- As project goes, will neeed to split code into multiple modules, then multiple files
- Packages can contain multiple binary crates and optionally one library crate, as packages grow, parts can be extracted into separate crates that become external dependencies
- Cargo also provides workspaces for large projects comprising of a set of interrelated packages that evolve together
- Can also encapsulate implementation details to reuse code at a higher level: once an operation has been implemented, other code can call this code via its public interface, treating it as a black box
- Can define which parts are public for other code to use and which are private implementation details, another way to limit amount of mental overhead
- Scope: the nested context in whcih code is written has a set of names that are defined as 'in scope'
   - Compilers need ot know whether a particular name at a particular spot refers to a variable, function, struct, etc and what the item means
   - Can create scopes and change which names are in or out of scopd, cannot have two items with the same name in the same scope; tools are available to resolve name conflicts
- Rust has features to manage code organization that control which details are exposed, private, and what names are in each scope of a program; these features encompass the module system
   - Packages: A Cargo feature that allows buildable, testable, and shareable crates
   - Crates: A tree of modules that produce a library or executeable
   - Modules and use: Allow programmer to control the organization, scope, and privacy of paths
   - Paths: A way of naming an item, such as a struct, function, or module
## Packages and Crates
### Crates
- Crates are the samllest amount of code that the Rust compiler considers at a time
   - Crates can contain modules and the modules may be defined in other files that get compiled witht eh crate
   - Crates can be either a binary crate or library crate
      - Binary crates are programs that compile to an executable that can run, such as a server or command line program
      - Each must have a function called `main` that defines that happens when the executable runs
      - Library crates don't have a `main` function and don't compile to an executable
      - They define functionality intended to be shared with multiple projects, for example `rand` crate provides functionaltiy that generates random numbers
      - Crate typically refers to a library crate and can be thought of interchangeably with libraries
- The crate root is a source file that the Rust compiler starts from and makes up the root module of a crate

### Packages
- A package is a bundle of one or more crates that provides a set of functionality
- Packages contain Cargo.toml files that describe how to build the crates
- Cargo is a package that contains the binary crate for the command line tool to build code, Cargo also contains a library crate that the binary crate depends on, other projects can depend on the Cargo library crate to use the same logic the Cargo command line tool uses
- A package can contain as many binary crates as possible but at most only one library crate
- A package must contain at least one crate
- After running `cargo new proj`
   - Cargo crates a Cargo.toml file, creating a package
   - src/main.rs has been created and is the Crate root of a binary crate, Cargo passes the crate root files to `rustc` to build the library or binary
   - If a package contains src/main.rs and src/lib.rs it both a binary and library crate, both with the same package
   - A package can have multiple binary crates by placing files winthe src/bin directory, each file will be a separate binary crate

## Defining Modules to Control Scope and Privacy
- Modules and paths allow item naming
- The `use` keyword brings a path into scope and the `pub` keyword to make items publci 

### Modules Cheat Sheet
- Start from the crate root: When compiling a crate, the compiler first looks in the crate root file for code to compile (src/lib.rs or src/main.rs)
- Declaring modules: In the create root file, new modules can be declared
   - "garden" module would be `mod garden;`, compiler will look for the module's code in the following places
      - Inline, within curly bracktes that replace the semicolon following `mod garden`
      - In the src/garden.rs
      - In the file src/garden/mod.rs
- Declaring submodules: In any file other than the crate root, submodules can be declared
   - Example: `mod vegetables` in src/garden.rs, compiler will look for the submodule's code within the directory named for the parent module in these places
      - Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
      - In the file src/garden/vegetables.rs
      - In the file src/garden/vegetables/mod.rs
- Paths to code in modules: once a module is part of a crate, it can refer to code in that module from anywher5e else in that same crate as long as privacy rules allow using the path to the code
   - A `Carrot` type in the garden vegetables module would be found at `crate::garden::vegetables::Carrot`
- Private vs public: Code within a module is private from its parent modules by default
   - To make a module public, declare it with `pub mod` instead of `mod`, to make items within a public module public as well, use `pub` before their declarations
- The `use` keyword: within a scope, the `use` keyword creates shortcuts to items to reduce the verbosity of repeated long paths
   - In any scope referring to `crate::garden::vegetables::Asparagus`, can create a shortcut with `use crate::garden::vegetables::Asparagus;` and then only need to write `Asparagus` to make use of that type in the scope
- Binary crate nanmed `backyard` with these rules:
   - ```
      backyard
      ├── Cargo.lock
      ├── Cargo.toml
      └── src
          ├── garden
          │   └── vegetables.rs
          ├── garden.rs
          └── main.rs```
### Grouping Related Code in Modules
- Modules allow code organization for readability and easy reuse but also allow the control of privacy of items because code within a module is privaye by default
- Can choose to make modules and the items within them public, exposing them to external code use and depend on them
- Can create new library crate with `cargo new <lib_name> --lib`
- Modules can also hold definitions for othe ritems such as structs, enums, constants, traits
- Using modules, can group related definitions together and name why they're related, also makes it easier to navigate and find new definitions relevant to these groups
- Reason for the term crate root is that the contents of the two files form a module named `crate` at the root of the crate's module structure, known as the module tree
- Example: ```
      crate
       └── front_of_house
           ├── hosting
           │   ├── add_to_waitlist
           │   └── seat_at_table
           └── serving
               ├── take_order
               ├── serve_order
               └── take_payment```
- This tree shows how some of the modules nest inside other modules, `hosting` nests inside `front_of_house`
- Some modules are also siblings like `hosting` and `serving` are siblings defined within `front_of_house`, if module A is defined within module B, it is the child of module B
- Entire module tree is rooted under the implicit module named `crate`

## Paths for Referring to an Item in the Module Tree
- To indicate to Rust where to find an item in a module tree, a path is used in the same way its used when navigating a filesystem, to call a function need to know its path
- Path can take two forms:
   - Absolute path: full path starting from crate root
      - For code from an external crate, absolute path begins with the crate name
      - For code from current crate, it starts with the literal `crate`
   - Relative path: starts from the current module and uses `self`, `super`, or an identifier in the current module
- Both path types are followed by one or more identifiers separated by double colons or `::`
- Choosing which path type depends on whether it is more likely to move item definition code separately from or together with the code that uses the item, based on what types of things may change, where the function is being called or if the actual function may be moved
- Preference is to specify absolute paths because it is more likely to move code definitions and the item calls independently of each other
- Everything is private by default, to make it private, place in a module
- Items in a parent module cannot use the private items inside child modeules but items in child modules can use the items in their ancestor  modules because child modules wrap and hide their implementation details but child modules can see the context in which they're defined
- Hiding implementation details is default, this way it is known which parts of the inner code that can be changed without breaking outer code, but Rust does give option to expose inner parts of child modules code to outer ancestor modules by using the `pub` keyword to make an item public

### Exposing Paths with the `pub` Keyword
- Need to publicize both modules and the inner functions in order to mark as public
- `pub` keyword on a module only lets code in its ancestor modules refer to it, not access its inner code
- Public API is a contract with users of crate that determines how they can interact with code

### Best Practices for Packages with a Binary and a Library
- Both crates will have the package name by default
- Typically, packages with this pattern of containing both a library and a binary crate will have just enough code in the binary crate to start an executable that calls code within the library crate, lets other projects beenfit from most of the functionality that the package provides because library crate's code can be shared
- Module tree shoudl be defined in src/lib.rs. Any public items can be used in the binary crate by starting pats with the name of the package
- Binary crate uses library crate just liek an external crate would use the library crate, can only use the public API
- Helps design a good API

### Starting Relative Paths with `super`
- Can construct relative paths in parent module, rather than the current module or crate root by using `super` at the start of the path
- Allows for referencing an item that is in the parent module that can make rearranging the module tree easier when the module is closely related to the parent but parent might be moved elsewhere in the module tree
   - Example: `super::deliver_order()`, goes to parent module

### Making Structs and Enums Public
- Can also use `pub` to make structs and enums public
- `pub` before a struct definiition makes the struct public but structs fields will still be private
- Can make each field public or not on a case-by-case basis
- If a struct has private fields, it must provide a public associated function that constructs an instance of it
- If making an eun public, all its varianbts are public, only need `pub` keyword before the `enum` keyword
- Structs are often useful without without their fields being public, so struct fields follow the general rule of everything being private unless annotated with `pub`

## Bringing Paths into Scope with the `use` Keyword
- Having to write out paths to call functions can be repetitive and inconvenient
- Can create shortcut with `use` keyword and use shorter name everywhere else within scope
- Example: `use crate::front_of_house::hosting;`
   - Then can reference child function with `hosting::add_to_waitlist()`
- Adding `use` is similar to creating a symbolic link in the filesystem
- Paths brought into scope with `use` also check privacy
- `use` only creates the shortcut for the particular scope in which the `use` occurs
   - Using an external `use` within a `mod` block would not work because the `use` is considered to be in an external scope

### Creating Idiomatic `use` Paths
- Idiomatic way to bring a function into scope with `use` is to bring the function's parent module into scope with `use` and specify the parent module when calling the function, making it clear that the function is not locally defined
- On the other hand, when bringig in structs, enums, and other items with `use`, it's idiomatic to specify the full path
   - Example with `HashMap` struct: `use std::collections::HashMap;`
- Exception to this is it two items with same name are brought into scope with `use` statements because Rust does not allow this
   - Example: ```
         use std::fmt;
         use std::io;
         fn function1() -> fmt::Result {}
         fn function2() -> io::Result<()> {}```
   - Parent modules distinguish the two `Result` types

### Providing New Names with the `as` Keyword
- Another solution to the problem of bringing two types of the same name ito the same scope with `use`: after the path, can specify `as` and a new local name or alias for the type
- Example: `use std::io::Result as IoResult;`, this reduces the name conflict between the `io::Result` and `fmt::Result` by aliasing `io::Result` to `IoResult`

### Re-exporting Names with `pub use`
- When bringing a name into scope with the `use` keyword, the name is private to the scope into which it was imported
- To enable external code outside that scope to refer to that name as if it had been defined in that scope, `pub` and `use` can be combined
- This is called re-exporting: bringing an item into scope but also making that item available for others to bring into their scope
- Example: `pub use crate::front_of_house::hosting` re-exports the `hosting` module from the root module, external code can use `restaurant::hosting::add_to_waitlist()` instead of `restaurant::front_of_house::add_to_waitlist()`
- Re-exporting is useful when the internal structure of code is different from how programmers calling the code would think about the domain
   - Restaurant would not be thought of as front of house and back of house by customers
- with `pub use`, can write code with one structure but expose a different structure, makes library well organized for programmers working on the library and programmers calling the library

### Using External Packages
- To add a package called `rand` to projet, added `rand = "0.9.0"` to Cargo.toml, tells Crgo to download the `rand` package and any dependencies from crates.io and make `rand` available to the project
- Then to bring `rand` definitions into the scope of the project, add a `use` line with `use rand::Rng`, starting with the name of the crate, `rand`, and listed the items we wanted to bring into scope, bringing the `Rng` trait into scope and call the `rand::thread_rng()` function
- Many packages available at crates.io, pulling them into a proejct involve the same steps: listing them into package's Cargo.toml file and using `use` to bring items from their crates into scope
- The standard `std` library is also a crat ethat is external to local package, but don't need to change Cargo.toml to include `std`, do need to refer to it with `use` to bring items from there into package's scope
   - For example, with `HashMap`, use line `use std::collections::HashMap;`, which is an absolute path starting with `std`, the name of the standard library crate

### Using Nested Paths to Clean Up Large `use` Lists
- If using multiple items defined in the same crate or same module, listing each item on its own line can take up a lot of vertical space in files
- Instead, can use nested paths to bring the same items into scope in one line by specifying ther common part of the path, followed by two colors, then curly braces arounb a list of the parts of the paths that differ
- Example: `use std::{cmp::Ordering, io};`
- Can use a nested path at any level in a path
   - Can include something directly with `self`
      - Example: `use std::io::{self, Write};`
   - Common part of these two paths is `std::io` which is enough to complete the first path, to merge the two paths into one use stateent, can use `self` in the nested path

### The Glob Operator
- To bring all public items defined in a path into scope, can specify that path followed by the `*` glob operator
- Example: `use std::collections::*;`
   - This `use` statement brings all public items defined in `std::collections` into the current scope
   - Glob can make it harder to tell what names are in scope and where a nameused in a program was defined
- Glob operator is often used when testing to bring everything under test into the `tests` module

## Separating Modules into Different Files
- When modules get larger, might want to move their definitions to separate files to make the code easier to navigate
- Can move all code inside of `mod front_of_house` into src/front_of_house.rs and place those modules there, keep `mod front_of_house`
- Only need ot load a file using a `mod` declaration once in the module tree, once compiler knows that the file is part of the project and knows where in the moduile tree the code resides because of where the `mod` statement was placed, other files in the project should refer to the loaded file's code using a path to where it was declared, note that mod is not an include operation
- Then moving `hosting`, change src/front_of_house.rs to only contain declaration of `hosting` module, then create a src/front_of_house directory and a hosting.rs to contain the definitions made in the `hosting` module

### Alternate File Paths
- For a module named `front_of_house` declared in the crate root, the compiler will look for the module's code in src/front_of_house.rs or src/front_of_house/mod.rs (older style but still supported path)
- For a module named `hosting` that is a submodule of `front_of_house`, the compielr will look for the modules code in: src/front_of_house/hosting.rs or src/front_of_house/hosting/mod.rs (older but still supported style), cannot use both styles for the same module, using mix of styles can be confusing, downside with using mod.rs is many files named mod.rs which can be confusing when open in editor at same time
- Note that `use` has no impact on what files are compiled as part of the crate, the `mod` keyword declares modules and Rust lokos in a file with the same name as the module for the code that goes into that module

### Summary
- Rust letsa package be split up into multiple crates and a crate into modules so they can refer to items defined in one module from another module
- These paths can be brought into scope with a `use` statement such that a shorter path for multiple uses of the item is used
- Module code is private by default but definitions can be made public by adding the `pub` keyword
