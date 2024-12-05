//! A svg target implementation for vector drawing output.

use std::future::Future;

use crate::{
    device::VDLProgram,
    errors::Result,
    ir::{AnimatableValue, IR},
};

/// A implementation of [`VDLProgram`] that output vector drawings as svg images.
#[allow(unused)]
pub struct SvgVDLProgram(Vec<IR>);

impl VDLProgram for SvgVDLProgram {
    type Output = String;

    type Execute<'a>
        = SvgGenerator<'a>
    where
        Self: 'a;
    fn execute<'a>(&'a self, variables: &'a [crate::ir::AnimatableValue]) -> Self::Execute<'a> {
        SvgGenerator::new(self, variables)
    }
}

#[allow(unused)]
pub struct SvgGenerator<'a> {
    program: &'a SvgVDLProgram,
    variables: &'a [AnimatableValue],
}

impl<'a> SvgGenerator<'a> {
    fn new(program: &'a SvgVDLProgram, variables: &'a [AnimatableValue]) -> Self {
        Self { program, variables }
    }
}

impl<'a> Future for SvgGenerator<'a> {
    type Output = Result<String>;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}
