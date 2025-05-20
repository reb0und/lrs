# Slices
- Slices allow referenes to a contiguous sequence of elements in a collection, rather than the entire collection, similar to `[1:4]`
   - This is a reference so it does not ahve ownership
- If to create a fucntion that takes a string of words separated by spaces and returns the first word in the string
   - If no space is found, return whole word
   - `fn first_word(s: &String) -> ?`, don't need ownership so a reference to a `String` is passed
   - Functions should not take ownership of arguments unless they need to
   - Could start by returning index at end of word
   - Convert element to bytes using `as_bytes`
   - Create an iterator over the byte array using the `iter` method
      - Yield the current count and the element during iteration `enumerate`
      - `enumerate` wraps the result of `iter` and returns each element as part of a tuple of current count and reference to each element instead
         - Can use patterns to destructure the tuple, `i` represents index in tuple, `&item` represents the single byte in the tuple
      - Space is represented by using the byte literal syntax `b' '`, if space is found, return the position, otherwise return length of the `String`
         - Could also return `String` slice from `[0:i]`
      - `String` could be modified and the result of `first_word` could no longer hold significance, for example, if the `String` cleared using `clear`, value could be gone, have to worry about indices gettign out of sync with the data

### String Slices
- A string slice is a reference to a part of a `String`
   - Reference to part of `String` exists in `[starting_index..ending_index]` with starting and ending indices, `ending_index` is one more than the last index in the slice
   - With Rust's range syntax: `..`, if starting at index 0, the first value can be dropped. `[..i]` and is equal to `[0..i`
   - If slice includes last byte of the `String`, the trailing number can be dropped `[3..len]` is the same as `[3..]`
   - Can also drop both values to take slice of entire String `[0..len]` = `[..]`
- String range indices must occur at valid UTF-8 character boundaries
- Type signifying string slice is `&str`
- After calling `first_word`, get a single value that is tied t othe underling data, made up of a reference to the starting point of slice and the number of elements in slice

### String Literals as Slices
- `let s = "hello world";` type of `s` here is `&str` which is a slice pointing to the specific point inside the binary, this is also why string literals are immutable, `&str` is an immutable reference

### String Slices as Parameters
- Since you can take slices of literals, not just `String` values, replace function signature from `s: &String` -> `s: &str`, such that it works with both `&String` and `&str` values
- If given a string slice, that can be passed directly, if given a `String`, can pass a slice of the `String` or a reference to the `String`
   - This takes advatage of deref coercions (review this)
- This makes code more general without losing any functionality
- Since `String`s are equivalent to whole slices of `String`s, can just pass a `String`

### Other Slices
- Can also take slice of arrays
- Slice can have type `&[i32]`
