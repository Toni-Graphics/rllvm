//! RLLVM's ir

pub mod compile;
pub mod var;
pub mod r#type;

macro_rules! IrTypeWith2 {
    ($name:tt, $trait:ident, $param1:tt, $param2:tt) => {
        pub struct $name<$param1, $param2> {
            pub inner1: $param1,
            pub inner2: $param2,
        }

        impl<$param1, $param2> $name<$param1, $param2> {
            /// Creates new instance
            #[allow(dead_code)]
            pub fn new(op0: $param1, op1: $param2) -> Box<Self> {
                Box::from(
                    Self {
                        inner1: op0,
                        inner2: op1,
                    }
                )
            }
        }

        #[allow(dead_code)]
        pub trait $trait<$param1, $param2>: crate::ir::compile::Compile {}
    };
}

macro_rules! IrTypeWith1 {
    ($name:tt, $trait:ident, $param1:tt) => {
        pub struct $name<$param1> {
            pub inner1: $param1,
        }

        impl<$param1> $name<$param1> {
            /// Creates new instance
            #[allow(dead_code)]
            pub fn new(op0: $param1) -> Box<Self> {
                Box::from(
                    Self {
                        inner1: op0,
                    }
                )
            }
        }

        #[allow(dead_code)]
        pub trait $trait<$param1>: crate::ir::compile::Compile {}
    };
}

pub mod ir {
    IrTypeWith2!(Add, AddTrait, T, U);
    IrTypeWith2!(Sub, SubTrait, T, U);
    IrTypeWith2!(Mul, MulTrait, T, U);
    IrTypeWith1!(Return, ReturnTrait, T);
}