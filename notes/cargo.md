# More About Cargo and Crates.io
- Customize build through release profiles
- Publish libraries on crates.io
- Organize large projects with workspaces
- Install binaries from crates.io
- Extend Cargo using custom commands

## Customizing Builds with Release Profiles 
- Release prilfes are predefined and customizeable profiles with different configurations that allow a programmer to have more control over various options for compiling code, each profile is configured independently of the others
- Cargo has two main profiles:
   - `dev` profile Cargo uses when running `cargo build`
      - Defined with good default for development
   - `release` profile Cargo uses when running `cargo build --release`
      - Good defaults for release builds
- Cargo has default settings for each of the profiles that apply when there are no explicitly added `[profile.*]` sections in the project's Cargo.toml file
   - By adding `[profile.*]` sections for any profile to customzie, can override any subset of the default settings
   - Example: ```
[profile.dev]
opt-level = 0```
   - `opt-level` setting controls the number of optiimizations Rustwill apply to code with a range of 0 to 3
   - Applying more optimizations extends compile time, in development should use fewer optimizations to compile faster even if resultant code runs slower
   - When releasing code, best to spend more time compiling
- Can override a default setting by adding a different value for it in Cargo.toml
- Can use optimization level 1 in development profile by setting `opt-level` to 1
- When running `cargo build`, Cargo will use the defaults for the `dev` profile plus customization to `opt-level`


## Publishing a Crate to Crates.io
- Crates registry at crates.io distributes the source code of packages
- Rust and Cargo have features that make published packages easier to find and use

### Making Useful Documentation Comments
- Accurately documenting packages will help users know how and when to use them, so its worth investing the time to write documentation
- Rust also has a particular kind of documentation, known as documentation comment, that will generate HTML documentation
   - HTML displays the contents of documentation comments for the public API items intended for programmers interested in knowing how to use the crate as opposed to know how crate is implemented
   - Documentation comments use three slashes `///`, instead of two and support Markdown notation for formatting the text, placed right before item they're documenting
   - Example: ```/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}```
   - Can generate the HTML documentation comment by running `cargo doc`, this command runs the `rustdoc` tool distributed with Rust and puts the generated HTML documentation in the target/doc directory
   - For convenience, running `cargo doc --open` will build the HTML for current crate's documentation (as well as other crate's dependencies) and open the documentation in a web browser

#### Commonly Used Sections
- Examples in `# Examples`
- Panics: scenarios in which the function documented could panic, callers of this function should make sure they don't call the code in these situations
- Errors: if the function produces a `Result`, describing these kinds of errors that might occur and what conditions might cause those rrors to be returned can be helpful to callers so they can write code to hanlde the different kinds of errors in different ways
- Safety: if the function is unsafe to call, there should be a section explaining why the function is unsafe and covering the invariants that the function expects callers to uphold
- Most documentation does not need all of these sections, but a good checklist

#### Documentation Comments as Tests
- Adding example code blocks in documentation comments can help demonstrate how to use library and doing so has an additional bonus: running `cargo test` will run the code examples in documentation as tests
- Should not have code that doesn't work because code has changed since the documentation was written

#### Commenting Contained Items
- `//!` adds documentation to the item that contains comments the comments rather than to the items following the comments, typically used inside the crate root file (src/lib.rs) by convention or inside a module to document the crate or the module as a whole
- To add documentation that describes the purpose of a crate, would add the documentation comments that start with `//!` to the beginning of the src/lib.rs file
- Example: ```
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--```
- There is no code after the last line that begins with `//!`, since starting the comments with `//!` instead of `///`, this documents the item that contains the comment rather than an item that follows this comment rather than an item that follows this comment, in this case, that item is the src/lib.rs file which is the crate root
- When running `cargo doc --open`, these comments will display on the front page of the documentation for the crate, above the list of public items in the crate
- Documentation commends within items are useful for describing crates and modules especially, good to explain the overall purpose of the contianer to help users understand the crate's organization

### Exporting a Convenient Public API with `pub use`
- Structure of a public API is a major consideration when publishing a crate, if people who use the crate are less familiar with the structure, they may have difficulty finding the pieces they want to use if crate has a large module hierarcy
- Can make items public using the `pub` keyword and bring items into scope with the `use` keyword, this may be inconvenient for users of a crate, finding a type used in a deep hierarchy may be difficult
- Solution is re-exporting items to make a public structure that's different from private structure by using `pub use`, re-exporting takes a public item in one location and makes it public in another location, as if it were defined in the other location instead
- To modify the internal organization of something, can modify crate to add `pub use` statements to re-export the items at the top level
- API documentation that `cargo doc` generates for this crate will now list and link re-exports on the front page, making the types and functions easier to find
- Crate users can still see and use the internal structure or they can use the more convenient structure
- In cases where there are many nested modules, re-exporting the types at the top level with `pub use` can make a significant difference in the experience of people who use the crate
- Another common use of `pub use` is to re-export definitions of a dependency in the current crate to make that crate's definition part of crate's public API
- Choosing `pub use` gives flexibility in how to structure a crate internally and decouple the internal structure from what is presented to users

### Adding Metadata to a New Crate
- Before publishing a crate, should add some metadata in the `[package]` section of the crate's Cargo.toml file
- Crate will need to be a unique name, while working on crate localy, can name it anything, crate names on crates.io are allocated on a first-come, first-served basis, once a crate name is taken, no one else can publish a crate with that name
   - Crate names can be set in the `name` field in the Cargo.toml file under the `[package]` section
   - Example: ```
[package]
name = "guessing_game"```
- Can use `cargo publish` to publish the crate at this point
- Also need to provide description and license
   - Description can be a sentence or two and all license identifiers can be found in the [SPDX](http://spdx.org/licenses/), can use `MIT`
   - Can use a license that doesn't appear in the SPDX by placing the text of that license in a file, including the file in the project, and using the `license-file` to specify the name of that file instead of using the `license` key
- Can use a dual license with `MIT OR Apache-2.0`, this demonstrates that multiple license identifiers can be specified by `OR` to have multiple licenses for a project
- Example: ```
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]```

### Publishing to Crates.io
- Publishing a crate uploads a specific version to crates.io for others to use
- Publishes are permanent and versions can never be overwritten, and code cannot be deleted, one goal of crates.io is to act as a permanent archive of code so that builds of all projects that depend on crates from crates.io will continue to work, allowing version deletions would make this goal impossible, no limit to the number of crate versions to be published

### Publishing a New Version of an Existing Crate
- After making changes to a crate and ready to release a new version, can change the version value specified in Cargo.toml file and republish, should use the [Semantic Versioning rules](http://semver.org) to decide what an appropriate next version number is based on the kinds of changes made, then run `cargo publish` to upload the new version

### Deprecating Versions from Crates.io with `cargo yank`
- Despite not being able to remove previous versions of a crate, can prevent any further projects from adding them as a new dependency, this is useful when a crate version is broken for some reason
- Yanking a version prevents new projects from depending on that version while allowing alle xisting projects that depend on it to continue
   - A yank means that all current projects with a Cargo.lock will not break and any future Cargo.lock files generated will not use the yanked version
   - To yank a version of the crate, in the directory of the crate, previously published, run `cargo yank` and specify which version to yank
   - Example: `cargo yank --vers 1.0.1`
   - By adding `--undo` to the command, can also undo a yank and allow projects to start depending on the version again
   - Example `cargo yank --vers 1.0.1 --yank`
   - Yank does not delete any code, cannot delete accidentally uploaded secrets, if that happens, should reset the secrets immediately

## Cargo Workspaces
- As a project develops, might find the library crate get bigger and want to split the package further into multiple library crates, Cargo offers feature called workspaces that can help manage multiple related packages that are developed in tandem

### Creating a Workspace
- A workspace is a set of packages that share the same Cargo.lock and output directory
- Cargo.toml file to configure entire workspace won't have a `[package]` file but will have a `[workspace]` section instead that will allow the addition of members to a workspace, can also use the latest and greatest version of Cargo's resolver algorithm by setting the `resolver` to `"3"`
- Example: ```
[workspace]
resolver = "3"```
- Can create a binary crate in the workspace by running `cargo run bin_crate` in the `bin_crate` directory
- Running `cargo new` inside a workspace also automatically adds the newly created package to the `members` key in the `[workspace]` definition in the workspace `Cargo.toml`
- Workspace has one target directory at the top level that the compiled artifacts will be placed into, member packages don't have their own target directories, even when running `cargo build` from inside the member directories would result in the compiled artifacts ending up in crate/target
- Cargo structures the target directory in a workspace like this because the crates in a workspace are meant to depend on each other, if each crate had its own target directory, each crate would have to recompile each of the other crates in the workspace to place the artifacts in its own target directory, by sharing the one target directory, the crates can avoid unnecessary rebuilding
- Note that each member has its own Cargo.toml file

### Creating the Seond Package in the Workspace
- Can have one package within one member depened on another member with a path dependency
- Example: ```
[dependencies]
add_one = { path = "../add_one" }```
- To run the binary crate from a specific directory, cap specify which package in the workspace to run using the `-p` argument and the package name with `cargo run`
- Example: `cargo run -p adder`
- Cargo does not assume that crates in a workspace will depend on each other, need to be explicit about the dependency relationships

#### Depending on an External Package in a Workspace
- Workspace will have only one Cargo.lock file at the top level rather than having a Cargo.lock in each crate's directory, ensuring that all crates are using the same version of all dependencies, if adding the `rand` package to multiple member crates, Cargo will resolve both of these to one version of `rand` and record that in one Cargo.lock
- Making all the crates in a workspace use the same dependencies, means the crates will always be compatible with each other
- Even though `rand` is used somewhere in the workspace, cannot use it in other crates in the workspace unless adding `rand` to their Cargo.toml files as well
- Despite this, no additional copies of `rand` will be downloaded
- Cargo will ensure that every crate in every package in the workspace using the `rand` package will use the same version as long as they specify compatible versions of `rand`, saving space and ensuring that the crates in the workspace will be compatible with each other
- If the creates in the workspace specify incompatible versions of the same dependency, Cargo will resolve each of them, but still try to resolve as few versions as possible

#### Addint a Test to a Workspace
- Running `cargo test` in a workspace will run the tests for all the crates in the workspace
- Can also run tests for a particular crate in a workspace from the top-level directory by using the `-p` flag and specifying the name of the crate to test
- If publishing the crates in the workspace to crates.io, each crate in the workspace will need to be published separately, can publish a particular crate in a workspace by using the `-p` flag and specifying the name of the crate to publish
- As projects grow, workspaces are useful, they enable working with smaller, easier-to-understand components than one big blob of code
- Keeping the crates in a workspace cna make coordination between crates easier if they are often changed at the same time

## Installing Binaries with `cargo install`
- The `cargo install` command allows the installation and use of binary crates locally, convenient way for developers to install tools that others have shared on crates.io
- Can only install packages that have binary targets, a runnable program that is created if the crate has a src/main.rs file or another file specified as a binary, as opposed to a library target that isn't runnable on its own but is suitable for including within other programs, usually crates have information in the README file about whether a crate is a library, has a binary target, or both
- All binary crates installed with `cargo install` are stored in the installation root's bin folder, $HOME/.cargo/bin
- For example, can install `ripgrep` with `cargo install ripgrep`

## Extending Cargo with Custom Commands
- If a binary in `$PATH` is `cargo-something`, can run it as if it were a Cargo subcommand by running `cargo something`
- Custom commands like this are also available when running `cargo --list`
