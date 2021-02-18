# Rosalind Test File Reader

Simple crate to help read test files from Rosalind.

## Example

```rust 
RosalindIO::from_path("tests/test-file.txt");
// or from 2 strings directly
RosalindIO::from_strings(
    String::from("Hello"), 
    String::from("World")
);
```

A file like this:

``` 
Input
Hello
Output
World
```

Becomes a RosalindIO struct like this:

```rust 
RosalindIO {
    input: "Hello",
    output: "World",
}
```