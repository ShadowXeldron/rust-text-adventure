// Global data. Time for my favourite past-time: getting into big arguments with the compiler!
use std::cell::Cell; // Intentionally not public

/* 
    Now, I'm sure you're probably wondering why I am using global data in the first place when it is intentional to the Rust programming language's deisgn that you cannot modify globals.
    Simply put, it's because this is a game. Not a web server, not a . It's a game, and global stuff is pretty much a requirement for games unless I want Clippy to have a go at me for using more than four arguments on a function.

    So, here's the globals:

    GOLD - A u32 variable
    PLAYERS - A slice containing multiple

    All of these are mutable.
*/