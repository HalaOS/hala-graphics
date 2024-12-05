//! The target device for vector drawing output.

use std::future::Future;

use crate::{
    errors::Result,
    ir::{AnimatableValue, IR},
};

/// The rendering target must implement this trait.
pub trait Device {
    /// Compile result.
    ///
    /// See [`VDLProgram`]
    type Program: VDLProgram;

    /// A future returns by [`compile`](Device::compile) function
    type Compile<'a>: Future<Output = Result<Self::Program>>
    where
        Self: 'a;

    /// Compile a new `VDL` program from ir codes.
    fn compile(&self, codes: Vec<IR>) -> Self::Compile<'_>;
}

/// A reference to a in-memory compiled `VDL` program.
pub trait VDLProgram {
    /// this `VDL` program executing output data.
    type Output;

    /// A future returns by [`execute`](VDLProgram::execute) function
    type Execute<'a>: Future<Output = Result<Self::Output>>
    where
        Self: 'a;

    /// Execute `VDL` program with animation variables.
    fn execute<'a>(&'a self, variables: &'a [AnimatableValue]) -> Self::Execute<'a>;
}
