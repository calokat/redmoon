# redmoon

A Rust implementation of Lua. **NOT PRODUCTION READY**. Mostly an excuse to hack with Rust and learn a lot.

## Will it ever be production ready?

That would be nice. I am thankfully at a point where it is easier to discuss what is missing versus what is here. However, the missing pieces are a pretty big deal:

- Garbage collection: This can be very hard to implement with Rust, since Rust has very strict rules regarding ownership. I have a solution in the works, but I won't know for sure until I start to work on it (which will likely not be for a while).

- Bytecode generation: Redmoon's current interpreter has no bytecode generation; Instead it interprets the syntax tree of the program. I am eager to start work on using proper bytecode. There's always more to learn!

- Goto statements and labels: This is a side effect of not using bytecode, meaning this feature will be very hard to implement as is. I plan to wait on adding goto's until I have a bytecode-based virtual machine.

- For loops: This one is kind of embarrassing. I should have this completed in the coming days or weeks, and hopefully I can delete this line and no one will know it was missing.

## Inspiration

A few months ago, I tried to compile Lua (written in C) to WebAssembly. I learned that this would be very difficult because Lua uses the `setjmp` and `longjmp` functions, which at the moment cannot be compiled to WASM. The only way to get Lua on the web was to use Emscripten, which simulates a POSIX environment with JavaScript and implements those functions in a very expensive way. The frustration I felt led me to consider reimplementing Lua in Rust, which has first-class WebAssembly support.

Starting this project also gave me excuses to finally learn three subjects in which I have long been interested: Lua, Rust, and compiler/interpreter development. Working on Redmoon made me a functioning Rust developer, and since Rust forced me to do things the "right" way, it probably made me a better developer in general.

## Prior Art

I owe a large debt to Robert Nystrom. His book [Crafting Interpreters](https://craftinginterpreters.com/) helped me wrap my head around basic concepts such as tokenization, parsing, and recursive descent. It's entertaining, well thought-out, and **free**! I would recommend this book to anyone interested in knowing how programming languages work. (Buy a copy if you can)

If you are looking for other projects like this, at the very least there's [Lua in Rust](https://github.com/cjneidhart/lua-in-rust). This project has a lot going for it that this one does not, like proper garbage collection, and its README has an extensive list of features (and soon-to-be features) that I have found helpful whenever I am measuring progress of my project.
