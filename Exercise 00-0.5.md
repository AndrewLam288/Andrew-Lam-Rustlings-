# Do Rustlings Sections 00 (Intro) to 05 (Vecs)

## Exercise 0.4 primitives type3

1. Create an array called `a` with fewer than 100 elements.
2. Check the length of the array.
3. If the array has 100 or more elements, print a message: "Wow, that's a big array!".
4. If the array has fewer than 100 elements, print a message: "Meh, I eat arrays like that for breakfast." and then panic with the message: "Array not big enough, more elements needed".

## Code Snippeet

fn main() {
    // Create an array called `a` with fewer than 100 elements in it.
    let a: [i32; 99] = [0; 99];
    // let a: [i32; 100] = [0; 100]; (my own test line for a have 100 elements)
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}

## My own write up

- The code didn't work at first when I tried to create an array without specifying the types. I initially wrote `let a: [99] = [0; 99];`. I wasn't familiar with the bit types and which type to use, but I'm starting to understand when to use which type now. I realized that I needed to specify the type as `i32` (32-bit integer).

- So what I change is I create a new let a line where we will create an array with fewer than 100 elements by let a: [i32; 99] = [0; 99]; ( Each element will be type 32 bit integer type, there will be 99 elements of 0's which is less than 100 elements )
: [i32; 99] will Specifies that a is an array of 99 elements, each of type i32.
[0; 99] indicates that 99 element will be 0.
  
- I note that rust terminal is pretty helpful when it also help me indicate what I'm missing or got wrong by just printing out 

error: expected type, found `99`
 --> exercises\04_primitive_types\primitive_types3.rs:3:13
  |
3 |     let a: [99] = [0; 99];
  |             ^^ expected type

error: could not compile `exercises` (bin "primitive_types3") due to 1 previous error

- I did try both having an array of 100 elements and less than 100 elements. The one with 100 elements work just fine but for less than 100 elements there are many things that appeared in the terminal which look pretty weird to me and make me thought that it didn't compile.

- Below are the things that appeared make me a bit confuse, I just wonder if these error line are necessary and also why is it happening because it is still compile.
  
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/9fc6b43126469e3858e2fe86cafb4f0fd5068869\library/std\src\panicking.rs:665
   1: core::panicking::panic_fmt
             at /rustc/9fc6b43126469e3858e2fe86cafb4f0fd5068869\library/core\src\panicking.rs:76
   2: primitive_types3::main
             at .\exercises\04_primitive_types\primitive_types3.rs:9
   3: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
             at C:\Users\lammi\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\ops\function.rs:250
   4: core::hint::black_box
             at C:\Users\lammi\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\hint.rs:389
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: process didn't exit successfully: `target\debug\primitive_types3.exe` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)