#  Setting up your environment


- Since the programming language comes with a variety of toolchains, tools, linkers, and compiler versions, choosing the best-fitting variation is not easy. Additionally, Rust works on all major operating systems—which adds another variable. 

- However, installing Rust has become a trivial task when using rustup (https://rustup.rs/). On the website, a helpful script (or installer on Windows) that takes care of retrieving and installing the required components can be downloaded. The same tool lets you switch between and update (and uninstall) these components as well. This is the recommended way.



    - Visual Studio Code (https://code.visualstudio.com/)
    - rustup (https://rustup.rs)
    - rustc (and the rest of the compiler toolchains)
    - cargo
    - RLS (short for Rust Language Server—this is for autocompletion)
    - Rust language support for Visual Studio Code
    
  
# Getting ready

- On a computer running either macOS, Linux, or Windows, only a web browser and internet connection are required. Bear in mind that the Windows installation works a little bit different from the *nix systems (Linux and macOS), which use scripts. 

# How to do it...

Each of the parts requires us to navigate to their respective websites, download the installer, and follow their instructions:
 
   -  Open the browser and navigate to (https://rustup.rs) and (https://code.visualstudio.com/)
   -  Choose the installers fit for your operating system.
   - After downloading, run the installers and follow their instructions, choosing the stable branches.
   - Once successfully installed, we'll go deeper into each installation.
   
   
   ```
   sangam$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

It will add the cargo, rustc, rustup and other commands to
Cargo's bin directory, located at:

  /Users/sangam/.cargo/bin

This can be modified with the CARGO_HOME environment variable.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /Users/sangam/.rustup

This can be modified with the RUSTUP_HOME environment variable.

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /Users/sangam/.profile
/Users/sangam/.bash_profile

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-apple-darwin
     default toolchain: stable
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>1

info: profile set to 'default'
info: default host triple is x86_64-apple-darwin
info: syncing channel updates for 'stable-x86_64-apple-darwin'
info: latest update on 2020-06-18, rust version 1.44.1 (c7087fe00 2020-06-17)
info: downloading component 'cargo'
  3.9 MiB /   3.9 MiB (100 %)   2.2 MiB/s in  2s ETA:  0s
info: downloading component 'clippy'
info: downloading component 'rust-docs'
 12.2 MiB /  12.2 MiB (100 %)   2.6 MiB/s in  4s ETA:  0s
info: downloading component 'rust-std'
 16.6 MiB /  16.6 MiB (100 %)   2.4 MiB/s in  7s ETA:  0s
info: downloading component 'rustc'
 56.1 MiB /  56.1 MiB (100 %)   2.0 MiB/s in 32s ETA:  0s
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 12.2 MiB /  12.2 MiB (100 %)   2.0 MiB/s in  5s ETA:  0s
info: installing component 'rust-std'
 16.6 MiB /  16.6 MiB (100 %)  13.7 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 56.1 MiB /  56.1 MiB (100 %)  10.5 MiB/s in  5s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable'

  stable installed - rustc 1.44.1 (c7087fe00 2020-06-17)


Rust is installed now. Great!

To get started you need Cargo's bin directory ($HOME/.cargo/bin) in your PATH
environment variable. Next time you log in this will be done
automatically.

To configure your current shell run source $HOME/.cargo/env
   
   
   
   
   ```
   # open Visual Studio Code
   
   
   - https://github.com/rust-lang/rls-vscode install rust extension
   - open extension setting and select rust extension 
   
   ![img](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/step1.png)
   
   -  enable cargo setting test following commands 
      
   ![img](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/select_extension_setting.png)
 ```
      
  rustc
Usage: rustc [OPTIONS] INPUT

Options:
    -h, --help          Display this message
        --cfg SPEC      Configure the compilation environment
    -L [KIND=]PATH      Add a directory to the library search path. The
                        optional KIND can be one of dependency, crate, native,
                        framework, or all (the default).
    -l [KIND=]NAME      Link the generated crate(s) to the specified native
                        library NAME. The optional KIND can be one of
                        static, framework, or dylib (the default).
        --crate-type [bin|lib|rlib|dylib|cdylib|staticlib|proc-macro]
                        Comma separated list of types of crates
                        for the compiler to emit
        --crate-name NAME
                        Specify the name of the crate being built
        --edition 2015|2018
                        Specify which edition of the compiler to use when
                        compiling code.
        --emit [asm|llvm-bc|llvm-ir|obj|metadata|link|dep-info|mir]
                        Comma separated list of types of output for the
                        compiler to emit
        --print [crate-name|file-names|sysroot|target-libdir|cfg|target-list|target-cpus|target-features|relocation-models|code-models|tls-models|target-spec-json|native-static-libs]
                        Compiler information to print on stdout
    -g                  Equivalent to -C debuginfo=2
    -O                  Equivalent to -C opt-level=2
    -o FILENAME         Write output to <filename>
        --out-dir DIR   Write output to compiler-chosen filename in <dir>
        --explain OPT   Provide a detailed explanation of an error message
        --test          Build a test harness
        --target TARGET Target triple for which the code is compiled
    -W, --warn OPT      Set lint warnings
    -A, --allow OPT     Set lint allowed
    -D, --deny OPT      Set lint denied
    -F, --forbid OPT    Set lint forbidden
        --cap-lints LEVEL
                        Set the most restrictive lint level. More restrictive
                        lints are capped at this level
    -C, --codegen OPT[=VALUE]
                        Set a codegen option
    -V, --version       Print version info and exit
    -v, --verbose       Use verbose output

Additional help:
    -C help             Print codegen options
    -W help             Print 'lint' options and default settings
    --help -v           Print the full set of options rustc accepts

 sangam$ rustc hello-world.rs
 sangam$ rustc hello-world
error: couldn't read hello-world: stream did not contain valid UTF-8

error: aborting due to previous error

 sangam$ cargo new hello-world
error: destination `/Users/sangam/Desktop/rustexample/hello-world` already exists

Use `cargo init` to initialize the directory
 sangam$ cargo init
     Created binary (application) package
sangam:Heroku_Go sangam$ cargo new hello-world
error: destination `/Users/sangam/Desktop/rustexample/hello-world` already exists

Use `cargo init` to initialize the directory
 sangam$ cargo new hello-world
     Created binary (application) `hello-world` package
 sangam$ cargo run
   Compiling Heroku_Go v0.1.0 (/Users/sangam/Desktop/rustexample/hello-world)
warning: crate `Heroku_Go` should have a snake case name
  |
  = note: `#[warn(non_snake_case)]` on by default
  = help: convert the identifier to snake case: `heroku_go`

warning: 1 warning emitted

    Finished dev [unoptimized + debuginfo] target(s) in 1.46s
     Running 
Hello, world!

      
  ```
  
# Working with the command line I/O

- The traditional way of communicating with the user on the command line is using standard streams. Rust includes helpful macros to deal with these simple cases. In this recipe, we will explore the basic workings of the classic Hello World program.

# How to do it...

  
-  Open a Terminal window (PowerShell on Windows) and run the `cargo new hello-world `command, which creates a new Rust project in a hello-world folder.
-  Once created, change into the directory with `cd hello-world` and open `src/main.rs` with a Visual Studio Code. The default code generated by cargo looks like this:
 
 ```
 fn main()
 {    
 println!("Hello, world!");
 }

 ```

