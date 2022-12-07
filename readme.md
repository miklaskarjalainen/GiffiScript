# GiffiLang
A simple interpreted toy language created as a programming challenge/exercise.

## Building
1. Install [Rust & Cargo](https://www.rust-lang.org/)
2. Clone this repository
3. type "cargo run --release"
4. executable can be found in the target directory, and a source file can be passed as an argument to the executable

## Example Code
Current state of the "programming language" is still very volatile and evolving.
But at the time of writing this, it looks like this.
~~~
fn sum(arg1, arg2) {
    print(arg1+arg2);
}

sum(5,5);
sum(9,6);
sum(10,25);
~~~

