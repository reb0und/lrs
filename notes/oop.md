# OOP Features
- OOP is a way of modeling programs

## Chracteristics of OOP Languages
- OOP languages share certain common characteristics, namely objects, encapsulation, and inheritance

### Objects Contain Data and Behavior
- OOP is defined in this way: OOP programs are made up of objects, an object packages both data and the procedures that operate on that data, the procedures are called methods or operations
- Using this definition, Rust is object oriented, structs and enums have data, and the `impl` blocks provide methods on structs and enums, even though structs and enums with methods aren't called objects, they provide the same functionality

### Encapsulation That Hides Implementation Details
- Another aspect of OOP is the idea of encapsulation, which means that the implementation details of an object aren't accessible to code using that object, therefore the only way to interact with an object is through its public API, code using the object shouldn't be able to reach into the object's internals and change data or behavior directly, this enables the programmer to change and refactor and object's internals without needing to change the code that uses the object
- Can use the `pub` keyword to decide which modules, types, functions and methods in code should be public, and by default, everything else is private
- Can define a struct `AveragedCollection` that has a field containing a vector of `i32` values, the struct can also have a field that contains the average of the values in the vector, meaning the average doesn't have to be computed on demand whenever it is needed
- `AveragedCollection` will cache the calculated average
- Example: ```
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}```
- The struct is marked `pub` so the other code can use it but the fields within the struct remain private, this is important to ensure that whenever a value is added or removed from the list, the average is also updated, this is done by implementing `add`, `remove`, and `average` methods
- Example: ```
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}```
- The public methods `add`, `remove`, and `average` are the only ways to access or modify data in an instance of `AveragedCollection`, when an item is added to the `list` using the `add` method or removed using the `remove` method, the implementations of each call the private `update_average` method handles updating the `average` field as well
- The `list` and `average` fields are left private, there is no way for external code to add or remove items from the `list` field directly, otherwise, the `average` feild might become out of sync whne the `list` changes, the `average` method returns the value in the `average` field, allowing external code to read the `average` but not modify it
- Since the implementation details of the struct `AveragedCollection` have been encapsulated, can easily change aspects, such as the data structure, can use a `HashSet<i32>` instead of a `Vec<i32` for the `list` field, as long as the signatures of the `add`, `remove`, and `average` public methods stayed the same, code using `AveragedCollection` wouldn't need to change, if the `list` was made public instead, this wouldn't necessarily be the case, `HashSet<i32>` and `Vec<i32>` have different methods for adding and removing items so the external code would likely have to change if it were modifying `list` directly
- If encapsulation is a required aspect for a language to be considered object oriented, Rust meets that requirement, the option to use `pub` or not for different parts of code enables encapsulation of implementation details

### Inheritance as a Type System and as Code Sharing
- Inheritance is a mechanism whereby an object can inherent elements from another object's definition, thus gaining the parent object's data nd behavior withuot having to define them again
- There is no way in Rust to define a struct that inherits the parent struct's fields and method implementations without using a macro
- Inheritance is used for two main reasons: reuse of code, can implement particular behavior for one type and inheritance enables reusing that implementation for a different type, can do this in a limited way in Rust using default trait method implementations, similar to a parent class having an implementation of a method and an inheriting child class also having the implementation of the method, can also override the default implementation of a trait's method when implementing that trait, similar to a child class overriding the implementation of a method from a parent class
- Other main reason to use inheritance relates to the type system: to enable a child type to be used in the same places as the parent type, this is also called polymorphism, means that can substitute multiple objects for each other at runtime if they share certain characteristics
- Polymorphism is thought of to some as synonymous with inheritance, but is more general concept that refers to code that can work with data of multiple types, for inheritance, types are generally subclasses
- Rust instead uses generics to abstract over different possible types and trait bounds to impose constrains on what those types must provide, this is sometimes called bounded parametric polymorphism
- Inheritance is often at risk of sharing mode code than necessary, subclasses shouln't always share all characteristics of their parent class but will do so with inheritance, can make a program's design less flexible, also introduces the possibility of calling methods on subclasses that don't make sense, this can make a program's design less flexible, and introduces the possibilty of calling methods on subclasses that don't make sense or that cause errors because the methods don't apply to the subclass, additionally, further restricting the flexibility of a program's design
- Trait objects enable polymorphism in Rust

## Using Trait Objects That Allow for Values of Different Types
- Vectors are limited by storing elements of only one type, can create a workaround by using enums of multiple variants that could be in a vector
- Sometimes want library user to be able to extend the set of types valid in a particular situation, will create example of GUI tool that ierates through a list of items, calling a `draw` method on each one to draw it to screen
- Will have types for people to use such as `Button` or `TextField`, in adition `gui` users will want to create their own types that can be drawn, example being an `Image` and another might be a `SelectBox`, `gui` needs to keep track of many values of different types and needs to call a `draw` method in each of these differently typed values, doesn't need to know exactly what will happen when the `draw` method is called, just that the value will have that method available to call
- To do this in a language with inheritance, might define a class named `Component` that has a method named `draw` on it, other clasess named `Button`, `Image`, and `SelectBox` would inherit from `Component` and thus inherit the `draw` method, they could each override the `draw` method to define their own behavior but hte framework could treat all of the types as it they were `Component` instances and call `draw` on them, since Rust doesn't have inheritance need another way to structure the `gui` library to allow users to extend it with new types

### Defining a Trait for Common Behavior
- To implement the behavior `gui` should have, need to define a trait named `Draw` that will have one method named `draw`, then can define a vector that takes a trait object
- A trait object points to both an instance of a type implementing the specified trait and a table used to look up trait methods on that type at runtime, can create a trait object by specifying some sort of pointer, such as an `&` reference of a `Box<T>` smart pointer, then the `dyn` keyword and specifying the relevant trait, can use trait objects im place of a generic or concrete type, wherever a trait object is used, Rust's type system will ensure at compile time that an yvalue used in that context will implement the trait object's trait, don't need to know all the possible types at compile tiem
- In Rust, refrain from calling structs and enums "objects" to distinguish them from other languages' objects, in a struct or enum, the data in the struct fields and the behavior in `impl` blocks are separated, whereas in other languages, the data and behavior combined into one concept is often labeled an object, however, trait objects are more like objects in other languages in the sense that they combine data and behavior, trait objects differ from traditional objects in that data can't be added to a trait object, trait objects aren't as generally useful as objects in other languages: their specific purpose is to allow abstraction across common behavior
- Example: ```
pub trait Draw {
    fn draw(&self);
}```
- This defines a struct named `Screen` that holds a vector named `components`, this vector is of type `Box<dyn Draw>`, which is a trait object, it's a stand-in for any type inside a `Box` that implements the `Draw` trait
- Example: ```
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}```
- On the `Screen` struct, will define a method named `run` that will call the `draw` method on each of its components
- Example: ```
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}```
- This works differently from defining a struct that uses a generic type parameter with trait bounds, a generic type parameter can be substituted with only one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime
- If wanting homogenous collections, using generics and trait bounds is preferable because the definitions will be monomorphized at compile time to use the concrete types
- On the other hand, with the method using trait objects, one `Screen` instance can hold a `Vec<T>` that contains a `Box<Button>`, as well as a `Box<TextField>`

### Implementing the Trait
- Can add some types that implement the `Draw` trait, will provide the `Button` type, a `Button` struct might have fields for `width`, `height`, and `label`
- Example: ```
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {}
}```
- The `width`, `height`, and `label` fields on `Button` differ from the fields on other components, for example, a `TextField` type might have the same fields plus a `placeholder` field, each type to drawn on screen will implement the `Draw` trait but will use different code in the `draw` method to define how to draw that particular type, as `Button` has here
- The `Button` type might have an additional `impl` block containing mehtods related to what happens when a user clicks the button, these methods won't apply to types like `TextField`
- If someone using library decides to implement a `SelectBox` struct that has `width`, `height`, and `options` fields, they would implement the `Draw` trait on the `SelectBox` type
- Example: ```
struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}```
- Library's user can now write their main function to create a `Screen` instance, to the `Screen` instance, they can add a `SelectBox` and a `Button` by putting each in a `Box<T>` to become a trait object, they can then call the `run` method on the `Screen` instance which will call `draw` on each of the components
- Example: ```
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("maybe"),
                    String::from("no"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height:10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}```
- When writing library, didn't know someone might add the `SelectBox` type but `Screen` implementation was able to operate on the new type and draw it because `SelectBox` implements the `Draw` trait which means it implements the `draw` method
- The concept of being concerned only with the messages a value responds to rather than the value's concrete type, is similar to the concept of duck typing in dynamically typed languages, if it walks like a duck and quacks like a duck, then it must be a duck, in the implementation of `run` on `Screen`, `run` doesn't need to know what the concrete type of each component is, it doesn't check whether a component is an instance of `Button` or a `SelectBox`, it just calls the `draw` method on the `component`, by specifying `Box<dyn Draw>` as the type of the values in the `components` vector, have defined `Screen` to need values that can call the `draw` method on
- The advantages of using trait objects and Rust's type system is to write cod similar to using duck typing is that there is never a need to check whether a value implements a particular method at runtime or worry about getting errors if a value doesn't implement a method but is called anyway, Rust won't compile the code if the values don't implement the traits that the trait objects need

### Trait Objects Perform Dyanmic Dispatch
- Compiler generates nongeneric implementations of functions and mehtods for each concrete type that is used in place of a generic type parameter, the code that results from monomporhization is doing static dispatch, which is when the compiler knows that method is intended to be called at compile time, this is opposed to dynamic dispatch when the compiler can't tell at compile time which method is called, in dynamic dispatch cases, the compiler emits code code that at runtime will figure out which method to call
- When using trait objects, Rust must perform dyanmic dispatch, the compiler doesn't know all the types that might be used with the code that's using trait objects, so it doesn't know which method implemented on which type to call, instead, at runtime, Rust uses the pointers inside the trait object to know which method to call, this lookup incurs a runtime cost that doesn't occur with static dispatch, dynamic dispatch also prevents the compiler from choosing to inline a method's code, which in turn prevents so e optimizations and Rust has some rules called dyn compatibility about where dynamic dispatch can and can't be used

## Implementing an Object-Oriented Design Pattern
- The state pattern is an object-oriented design pattern, the crux of the pattern is to define a set of states a value can have internally, the states are represented by a set of state objects and the value's behavior changes based on its state, will work through blog post struct that has a field to hold its state, which will be a state object from the set "draft", "review", or "published"
- State objects share funcionality, in Rust, can use structs or traits, rather than objects or inheritance, each state object is responsible for its own behavior and governing when it should change into another state, the value that holds a satate object knows nothign about the different behavior of the states or when to transition between states
- Advantage of the state pattern is that when the reqruirements of the program change, won't need to change the code of the value holding the state or the code that uses the value, will only need to update the code inside one of the state objects to change its rules or perhaps add more state objects
- Will implement the state pattern in a more traditional OOP way, then will use an approach that is more natural in Rust
- Final functionality for incrementally implementing a blog post workflow using the state pattern should look like:
    - A blog post starts with an empty draft
    - When the draft is done, a review of the post is requested
    - When the post is approved, it gets published
    - Only published blog posts return content to print so unapproved posts can't accidentally be published
- Any other changes attempted on a post should have no effect, for example, if trying to approve a draft blog post before a requesting a review, the post should remain an unpublished draft
- The following code is an example of the workflow in code form
- Example: ```
fn main() {
    let mut post = Post::new();

    post.add_text("xyz");
    assert_eq("", post.content());

    post.request_review();
    assert_eq("", post.content());

    post.approve();
    assert_eq("", post.content());
}```
- Want to allow user to create a new draft blog post with `Post::new`, want to allow text to be added to the blog post, if trying to get the post's content immediately, before approval, shouldn't get any text because the post is still a draft, have added `assert_eq!` in the code for demo purposes, a good unit test would be to assert that a draft blog post returns an empty string from the `content` method
- Next, want to enable a request for a review of the post and want `content` to return an empty string while waiting for the review, when the post receives approval, it should get published, meaning the text of the post will be returned when `content` is called
- The only type interacted with is the `Post` type, this type will use the state pattern and will hold a value that will be one of three state objects representing the various states a post can be in, draft, review, or published, changing from one state to another will be managed internally within the `Post` type, the states change in response to the methods called by library's users on the `Post` instance, but they don't have to manage the state changes directly, also users can't make a mistake with the states such as publishing a post before it's reviewied

### Defining `Post` and Creating a New Instance in the Draft State
- Need a public `Post` struct that holds some content, will start with the definition of the struct and an associated public `new` function to create an instance of `Post`, will also make a private `State` trait that will define the behavior that all state objects for a `Post` must have
- Then `Post` will hold a trait object of `Box<dyn State>` inside an `Option<T>` in a private field named `state` to hold the state object
- Example: ```
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}```
- The `State` trait defines the behavior shared by different post states, the state objects are `Draft`, `PendingReview`, and `Published` and they will all implement the `State` trait, now trait doesn't have any methods
- `Draft` state is state that post starts in
- When creating a new `Post`, set its `state` field to a `Some` value that holds a `Box`, this `Box` points to a new instance of the `Draft` struct, this ensures that whenever a new instance of `Post` is created, it will start out as a draft, since the `state` of a `Post` is private, there is no way to create a `Post` in any other state, in the `Post::new` function, can set the `content` field to a new empty `String`

### Storing the Text of the Post Content
- Want to be able to call a method named `add_text` and pass it a `&str` that is then added as the text content of a blog post, this can be implemented as a method, rather than exposing the `content` field as `pub`, so that later can implement a method that will control how the `content` field's data is read, the `add_text` method is straightforward
- Example: ```
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }```
- The `add_text` method takes a mutable reference to `self` because the `Post` instance is changed that `add_text` is called on, then called `push_str` on the `String` in `content` and pass the `text` argument to add the saved `content` this behavior doesn't depend on the state the post is in, and is not part of the state pattern, the `add_text` method does not interact with the `state` field at all

### Ensuring the Content of a Draft Post is Empty
- Even after calling `add_text` and added some content to the post, still want the `content` method to return an empty string slice since the post is still in the draft state
- Current content method is implemented with simplest thing to fulfill this requirement, always returning an empty string slice, to be changed later once implementing the ability to change a post's state so it can be published, posts can only be in draft state currently so post content should always be empty
- Example: ```
    pub fn content(&self) -> &str {
        ""
}```
### Requesting a Review Changes the Post's State
- Need to add functionality to request a review of a post, which should change its state from `Draft` to `PendingReview`
- Example: ```
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}```
- `Post` is given a public method named `request_review` that will take a mutable reference to `self`, and then call an internal `request_review` method on the current state of `Post`, and this second `request_review` method consumes the current state and returns a new state
- Added the `request_review` method to the `State` trait, all types that implement the trait now need ot implement the `request_review` method, instead of having `self`, `&self`, or `&mut self` as the first parameter of the method, have `self: Box<Self>`, this means the method is only value when called on a `Box` holding the type, this takes ownership of `Box<Self>` and invalidates the old state so the state value of the `Post` can transform into a new state
- To consume the old state, the `request_review` method needs to take ownership of the state value which is where the `Option` in the `state` field of `Post` comes in, can call the `Take` method to take the `Some` value out of the `state` field and leave a `None` in its place because Rust doesn't allow unpopulated fields in structs, this allows moving the `state` value out of `Post` rather than borrowing it, can then set the post's `state` value to the result of the operation
- Need to set the `state` to `None` temporarily rather than setting it directly with code like `self.state = self.state.request_review();` to get ownership of the `state` value, this ensures `Post` can't use the old `state` value after transforming into a new state
- The `request_review` method on `Draft` returns a new, boxed instance of a new `PendingReview` struct, which represents the state when a post is waiting for a review, the `PendingReview` struct also implements the `request_review` method but doesn't do any transformations, rather it returns itself because when requesting a review on a post already in the `PendingReview` state, it should stay in the `PendingReview` state
- The `request_review` method on `Post` is the same no matter its `state` value, each state is responsible for its own rules
- Will leave the `content` method on `Post` as is, returning an empty string slice, can now have `Post` in the `PendingReview` state as well as in the `Draft` state

### Adding `approve` to Change the Behavior of `content`
- The `approve` method will be similar to the `request_review` method, will set `state` to the value that the current state says it should have when the state is approved
- Example: ```
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        ""
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}```
- Similar to the way `request_review` on `PendingReview` works, if calling the `approve` method on a `Draft` it will have no effect because `approve` will return `self`, when calling `approve` on a `PendingReview` it returns a new, boxed instance of the `Published` struct, the `Published` struct implements the `State` trait, and for both the `request_review` method and the `approve` method, it returns itself, because the post should stay in the `Published` state in those cases
- Now need to update the `content` method on `Post`, want the value returned from `content` to depend on the current state of `Post`, need the `Post` to delegate a `content` method defined on its `state`
- Example: ```
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }```
- Since the goal is to keep these rules inside the structs that implement `State`, call a `content` method on the value in `state` and pass the post instance (`self`) as an argument, then return the value that's returned from using the `content` method on the `state` value
- Call the `as_ref` method on the `Option` because want a reference to the value inside the `Option` rather than ownership of the value, since `state` is an `Option<Box<dyn State>>>`, since `state` is an `Option<Box<dyn State>>`, can call `as_ref`, an `Option<&Box<dyn State>>` is returned, not calling `as_ref` would result in an error because can't move `state` out of the borrowed `&self` function parameter
- Then call `unwrap` method which will never panic because the methods on `Post` ensure that the `State` always contains a `Some` value when those methods are done
- At this point, when calling `content` in the `&Box<dyn State>`, deref coercion will take effect on the `&` and the `Box` so the `content` will ultimately be called on the type that implements the `State` trait, that means need to add `content` to the `State` trait definition and need to add logic for what content to return depending on which state there is
- Example: ```
fn main() {}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str { "" }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}```
- Added a default implementation for the `content` method that returns an empty string slice, means that don't need to implement `content` on the `Draft` and `PendingReview` structs, the `Published` struct will override the `content` method and return the value in `post.content`
- Need lifetime annotations on this method, taking a reference to a `post` as an argument and returning a reference to part of that `post`, so the lifetime of the returned reference is related to the lifetime of the `post` argument
- Logic related rules live in the state objects rather than being scattered throughout `Post`

### Why Not an Enum
- One disadvantage of using an enum is  that every place that checks the value of the enum will need a `match` expression or similar to handle every possible variant, this could be more repetitive than the trait object solution

### Trade-offs of the State Pattern
- Rust is capable of implementing the OOP state pattern to encapsulate different kinds of behaviors a post should have in each state, the methods on `Post` know nothing about the various behaviors, only have to look in one place to know the different ways a published post can behave, the implementation of the `State` trait on the `Published` struct 
- If creating an alternative implementation that didn't use the state pattern. may need to use `match` expressions in the methods on `Post` or even in the `main` code that checks the state of the post and changes behavior in those places, this would mean that would have to look in several places to understand all the implications of a post being in the published state, this would only increase the more states added, each of those `match` expressions would need another arm
- With the state pattern, the `Post` methods and the palces using `Post` don't need match expressions and to add a new state, would only need to add a new struct and implement the trait methods on that one struct
- The implementation using state pattern is easy to extend to add more functionality, to see the simplicity of maintaining code that uses the state pattern:
    - Add a `reject` method that changes the post's state from `PendingReview` back to `Draft`
    - Require two calls to `approve` before the state can be changed to `Published`
    - Allow users to add text only when a post is in the `Draft` state, have the state object responsible for what might change about the content but not responsible for modifying the `Post`
- One downside of the state pattern is that, since the states implement the transitions between states, some of the states are coupled to each other, adding another state between `PendingReview` and `Published` such as `Scheduled` would require changing the code in `PendingReview` to transition to `Scheduled` instead, would be less work if `PendingReview` didn't need to change with the addition of a new state, but would mean switching to another design pattern
- Another downside is that some logic has been duplicated, to eliminate this duplication, can try to make default implementation for the `request_review` and `approve` methods on the `State` trait that return self, however this wouldn't work, when using `State` as a trait object, the trait doesn't know what the concrete `self` will be exactly so the return type isn't known at compile time (this is one of the dyn compatability rules)
- Other duplicaton includes the similar implementations of the `request_review` and `approve` methods on `Post`, both use `Option::take` with the `state` field of `Post`, if `state` is `Some`, they delegate to the wrapped value's implementation of the same method and set the new value of the `state` field to the result, if there were a lot of methods on `Post` following this pattern, mau consider defining a macro to eliminate the repition
- By implementing the state pattern exactly as it's defined for OOP languages, not taking full advantages of Rust's strengths

### Encoding States and Behaviors as Types
- Rather than encapsulating the states and transitions completely so outside code has no knowledge of them, can encode the states into different types, Rust's type checking system will prevent attempts to use draft posts where only published posts are allowed by issuing a compiler error
- Still enable the creation of new posts in the draft state using `Post::new` and the ability to add text to the post's content, but instead of having a `content` method on a draft post that returns an empty string, can make it so draft posts don't have the `content` method at all, this way, if getting a draft post's content, will get a compiler error stating that the method does not exist, as a result will be impossible to accidentally display draft post content in production because that code won't compile
- Example: ```
pub struct Post {
    content: String,
}

pub Struct DraftPost {
    content: String,
}

impl Post {
    fn new() -> Post {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}```
- Both the `Post` and `DraftPost` structs have a private `content` field that sotres the blog post text, the structs no longer have the `state` field since the encoding of the state is moved to the types of the structs, the `Post` struct will represent a published post and it has a `content` method that returns the `content`
- Still have a `Post::new` function but instead of returning an instance of `Post` it returns an instance of `DraftPost`, since `content` is private and there are no functions that return `Post`, it is impossible to create an instance of `Post` now
- The `DraftPost` struct has an `add_text` method to add text to `content` as before but note that `DraftPost` does not have a `content` method defined, now the program ensures all posts start as draft posts and draft posts don't have their content available for display, any attempt to get around these constraints results in a compiler error

#### Implementing Transitions as Transformations into Different Types
- How to get a published post? Can enforce the rule that a draft post has to be reviewed and approved before it can be published, a post in the pending review state should still not display any content, can implement these constraints by adding another struct `PendingReviewPost`, defining the `request_review` method on a `DraftPost` to return a `PendingReviewPost` and defining an `approve` method on `PendingReviewPost` to return a `Post`
- Example: ```
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content
        }
    }
}```
- The `request_review` and `approve` methods take ownership of `self`, consuming the `DraftPost` and `PendingReviewPost` instances and transforming them into a `PendingReviewPost` and a published `Post` respectively, this way, won't have any lingering `DraftPost` instances after calling `request_review` on them and so forth, the `PendingReviewPost` struct doesn't have a content method defined on it so attempting to read its content results in a compiler error, as with `DraftPost`, since the only way to get a published `Post` instance that does have a `content` method is to call the `approve` method on a `PendingReviewPost` and the only way to get a `PendingReviewPost` is to call the `request_review` method on a `DraftPost`, now encoded the blog post workflow into the type system
- Also have to make some small changes to `main`, the `request_review` and `approve` methods return new instances rather than modifying the struct they're called on, so need to add more `let post = ` shadowing assignments to save the returned instances, also can't have the assertions about the draft and pending review posts' contents be empty strings, nor are they needed, can't ocmpile code that tries to use the content of posts in thoe states any longer, updated code is as follows:
- Example: ```
fn main() {
    let mut post = Post::new();

    post.add_text("xyz");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("xyz", post.content());
}```
- The changes made to `main` to reassign `post` mean that this implementation doesn't quite follow the OP state pattern anymore, the transformations between the states are no longer encapsulated entirely within the `Post` implementation, gain is that invalid states are now impossible because of the type system and the type checking that happens at compile time, this ensures that certain bugs such as display of the content of an unpublished post will be discovered before they make it to production
- Even though Rust is capable of implementing OOP design patterns, other patterns such as encoding state into the type system are also available in Rust, these patterns have different trade-offs, may need to be very familiar with OOP patterns, rethinking the problem to take advantage of Rust's features can provide benefits such as preventing some bugs at compile time, OOP patterns won't always be the best solution in Rust due to certain features like ownership that OOP languages don't have

### Summary
- Can use trait objects to get some OOP features in Rust, dynamic dispatch can give code some flexibility in exchange for a bit of runtime performance, can use this flexibility to implement OOP patterns that can help code's maintainability, Rust also has other features like ownership that OOP languages don't have, an OOP pattern won't always be the best way to take advantage of Rust's strengths but is an available option
