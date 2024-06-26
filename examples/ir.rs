use std::error::Error;
use rllvm::prelude::*;

fn main() -> Result<(), Box<dyn Error>>{
    let mut contxt = Context::new( Triple::host() )?;
    let func = contxt.add_function("main", vec![], Type::u32);

    func.ir.push( Return::new(5) );

    unsafe {
        let mut func: JitFunction<unsafe extern "C" fn() -> u32> = contxt.get_jit_function("main")?;
        let out = func.call();

        println!("main() -> {}", out);

        assert_eq!(out, 5);
    }

    Ok(())
}