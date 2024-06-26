# RLLVM

![Languages](https://img.shields.io/github/languages/top/Toni-Graphics/rllvm?logo=rust)
![GitHub Repo stars](https://img.shields.io/github/stars/Toni-Graphics/rllvm?style=flat)
![GitHub License](https://img.shields.io/github/license/Toni-Graphics/rllvm)
![Dynamic TOML Badge](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FToni-Graphics%2Frllvm%2Fmain%2FCargo.toml&query=%24.package.version&label=version)

LLVM alternativ

## Example

```rust
use std::error::Error;
use rllvm::prelude::*;

fn main() -> Result<(), Box<dyn Error>>{
    let mut contxt = Context::new( Triple::host() )?;
    let func = contxt.add_function("add", vec![Type::u32, Type::u32], Type::u32);
    let asm = func.asm_func()?;

    let x = asm.arg(0).unwrap();
    let y = asm.arg(1).unwrap();

    func.ir.push( Return::new(*(x + y) ) );


    unsafe {
        let mut func: JitFunction<unsafe extern "C" fn(u32, u32) -> u32> = contxt.get_jit_function("add")?;
        let out = func.call(5, 5);

        println!("main() -> {}", out);
    }

    Ok(())
}
```

## ToDo

Here is a bit of ToDo for my libary:

## v0.1.2

- [x] Starting high level ir struct
  - [x] Use traits `impl Compiler for Ir::Add<Var, Int>` so i can overload the enum variants
  - [x] Make it compilable
  - [ ] Implement `mov`, `add`, `sub`, `mul`, `div` | `ints`, `floats`
    - [ ] mov
    - [x] add
    - [x] sub
    - [x] mul
    - [ ] div
    - [x] ints
    - [ ] floats
  - [x] Starting high level ir builder
    - [x] via traits

## v0.1.3

- [x] Implement `args` to the high level ir
- [x] Add option (in `context`) to compile to object file
- [ ] Naming convention
  - [x] generate
  - [ ] parse
  