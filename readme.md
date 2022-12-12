# GiffiLang
A simple interpreted toy language created as a programming challenge/exercise.

## Building
1. Install [Rust & Cargo](https://www.rust-lang.org/)
2. Clone this repository
3. type "cargo run --release"
4. executable can be found in the target directory, and a source file can be passed as an argument to the executable

## Example Code
Current state of the "programming language" is still very volatile and evolving.

An implementation of FizzBuzz in GiffiScript.
~~~js
import "io"; // needed for 'print()'

let idx = 0;
while idx < 100 {
    idx = idx + 1;
    if idx % 15 == 0 {
        print("FizzBuzz");
    }
    else if idx % 3 == 0 {
        print("Fizz");
    }
    else if idx % 5 == 0 {
        print("Buzz");
    }
    else {
        print(idx);
    }    
}
~~~

