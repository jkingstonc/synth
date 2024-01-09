# synth
Sythn programming language

# Goals
The point of synth is to make a programming languge that gets the job done without the language/compiler getting in your way. that is all.

- Minimal syntax where not needed (i.e. minimal syntax for repeated code such as variable declerations, but for function arguments you must specify the names)
- Low friction systems programming language
- Safe (ish... make it hard to shoot yourself in the foot and along the way we will tell you if you are about to)
- No coloured functions/procedures
- Cross-compilation
- Hot-reloading
- Standard package manager
- Plenty of built-in debugging
- No weird exception control flow

# Building

## Windows
Currently Synth uses LLVM (... in the future hopefully not). To build LLVM, you will need
- Perl (`winget install -e --id StrawberryPerl.StrawberryPerl`)
- C/C++ Compiler (I reccomend MinGW)
- Cmake

To build LLVM, Synth comes with `llvmenv` which is a crate to manage LLVM instances. First run `llvmenv init`. Then `llvmenv entries` to find an entry. Then `llvmenv build-entry <version>` to build that entry (warning: this can take a VERY long time so go and make a coffe/tea and sit back and relax whilst Cmake works it's sweet magic).

Once LLVM is installed on your system, you are good to go! You can run `cargo build <options>`

# Running
`cargo run -- --file .\examples\dev.sy --arch x86 -o 1 --write-ir true` 

# Resources
- https://github.com/jkingstoncsecond/trove

# TODO
- [ ] Parse
- [ ] Type analysis
- [ ] IR generation
    - [ ] if not used, dont generate
- [ ] X86 generation

# Bugs
- [ ] need a newline at the end otherwise lexer breaks
- [ ] when we encounter a keyword, we don't check if theres whitespace after it which we should
- [ ] cannot comp on a single value i.e. `comp 0`
