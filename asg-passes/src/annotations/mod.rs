// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

pub mod annotation;
pub use annotation::*;

mod always_const;
use always_const::*;

mod core_function;
use core_function::*;

use leo_asg::*;
use leo_errors::Result;

// To resolve below issues should we consider moving annotation handling to AST pass level?
// But we currently have only one annotation that does this, and its a temporary measure till we can do better approach.
// Will we have other annotations that need to be done pre type-inference?
fn handle_annotations<'a>(function: &'a Function<'a>) {
    for (name, annotation) in function.annotations.iter() {
        match name.as_str() {
            "AlwaysConst" => AlwaysConst::resolve(function), // @gluax this change needs to be done before hitting the ASG though.
            "CoreFunction" => CoreFunction::resolve((function, annotation)),
            _ => {}
        };
    }
}

pub struct AnnotationResolver {}

impl<'a> ReconstructingReducerExpression<'a> for AnnotationResolver {}

impl<'a> ReconstructingReducerProgram<'a> for AnnotationResolver {
    fn reduce_function(&mut self, function: &'a Function<'a>, body: Option<&'a Statement<'a>>) -> &'a Function<'a> {
        function.body.set(body);
        handle_annotations(function);
        function
    }

    fn reduce_circuit_member_function(
        &mut self,
        _: CircuitMember<'a>,
        function: &'a Function<'a>,
    ) -> CircuitMember<'a> {
        handle_annotations(function);

        CircuitMember::Function(function)
    }
}

impl<'a> ReconstructingReducerStatement<'a> for AnnotationResolver {}

impl<'a> AsgPass<'a> for AnnotationResolver {
    type Input = Program<'a>;
    type Output = Result<Program<'a>>;

    fn do_pass(asg: Self::Input) -> Self::Output {
        let pass = AnnotationResolver {};
        let mut director = ReconstructingDirector::new(asg.context, pass);
        Ok(director.reduce_program(asg))
    }
}
