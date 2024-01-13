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
### Installing LLVM
WARNING: this may make you want to rip your hair out...

- First install Visual Studio and install: Visual Studio Core Editor, Desktop Development With C++, MSVC & Windows 11 SDK 
- First clone LLVM `git clone https://github.com/llvm/llvm-project.git --branch release/17.x llvm` where the release is your release number
- Then setup your build directory inside `cd llvm && mkdir build && cd build`
- Now, open Visual Studio and open a powershel dev terminal `View -> Terminal`
- From within that terminal, generate the project files `cmake -DLLVM_ENABLE_PROJECTS=all ../llvm`
- Then build & install it `cmake --build . --target install --config Release`
- This will probably build LLVM in the `Release` directory in your build folder, put it somewhere nice like `c:\llvm\Release`
- Add `c:\llvm\Release\bin` to the path
- Set the env var (for whatever version you're using) `LLVM_SYS_170_PREFIX` to `c:\llvm`
- It should work! (it likely won't)
- In the case it doesn't you may have to copy some of the missing header files from your current `build` directory to the new build you've put in `c:\llvm`

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
