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

use std::cell::Cell;

use leo_asg::*;
use leo_errors::{Result, emitter::Handler};

pub struct ConstantFolding<'a, 'b> {
    program: &'b Program<'a>,
    handler: &'b Handler,
}

impl<'a, 'b> ExpressionVisitor<'a> for ConstantFolding<'a, 'b> {
    fn visit_expression(&mut self, input: &Cell<&'a Expression<'a>>) -> VisitResult {
        let expr = input.get();
        // TODO @gluax, @egregius313 Implement results
        match expr.const_value() {
            Ok(Some(const_value)) => {
                let folded_expr = Expression::Constant(Constant {
                    parent: Cell::new(expr.get_parent()),
                    span: expr.span().cloned(),
                    value: const_value,
                });
                let folded_expr = self.program.context.alloc_expression(folded_expr);
                input.set(folded_expr);
                VisitResult::SkipChildren
            },
            Ok(None) => VisitResult::VisitChildren,
            Err(e) => {
                self.handler.emit_err(e);
                VisitResult::VisitChildren
            },
        }
    }
}

impl<'a, 'b> StatementVisitor<'a> for ConstantFolding<'a, 'b> {}

impl<'a, 'b> ProgramVisitor<'a> for ConstantFolding<'a, 'b> {}

impl<'a, 'b> AsgPass<'a, 'b> for ConstantFolding<'a, 'b> {
    fn do_pass(handler: &'b Handler, asg: Program<'a>) -> Result<Program<'a>> {
        let pass = ConstantFolding { program: &asg, handler };
        let mut director = VisitorDirector::new(pass);
        director.visit_program(&asg).ok();
        Ok(asg)
    }
}
