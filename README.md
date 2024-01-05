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

# Running
`cargo run -- --file .\examples\dev.sy --arch x86 -o 1 --write-ir true`

# Resources
- https://github.com/jkingstoncsecond/trove

# TODO
- Parse [-]
- Type analysis [-]
- IR generation [-]
- X86 generation [-]