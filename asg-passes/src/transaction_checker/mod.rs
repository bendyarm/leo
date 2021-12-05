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
use leo_errors::{emitter::Handler, Result};

use snarkvm_dpc::{network::testnet2::Testnet2, Network};

pub struct TransactionChecker<'a, 'b> {
    program: &'b Program<'a>,
    handler: &'b Handler,
    count: u8,
}

impl<'a, 'b> ExpressionVisitor<'a> for TransactionChecker<'a, 'b> {}

impl<'a, 'b> StatementVisitor<'a> for TransactionChecker<'a, 'b> {}

impl<'a, 'b> ProgramVisitor<'a> for TransactionChecker<'a, 'b> {
    fn visit_function(&mut self, input: &'a Function<'a>) -> VisitResult {
        if input.annotations.keys().any(|k| k == "transition".to_string()) {
            /// Cannot have more than allowed number of transitions for a given program
            if self.count == Testnet2::NUM_TRANSITIONS {
                todo!();
                self.handler.emit_err();
            }

            /// Check that function parameters have the appropriate types and increment count
            if input.arguments.len() > 2 {
                todo!();
                self.handler.emit_err();
            }

            match input.output {
                Type::Tuple(_) => {
                    // Check that length of tuple is less than 2
                    // Check that elements of the tuple (if present) are stdlib Records
                    todo!();
                }
                Type::Circuit(cir) => {
                    todo!();
                    // Check that circuit is a stdlib Record
                }
                _ => {
                    todo!();
                    self.handler.emit_err()
                }
            }
            self.count += 1;
        }

        VisitResult::SkipChildren
    }
}

impl<'a, 'b> AsgPass<'a> for TransactionChecker<'a, 'b> {
    type Input = (Program<'a>, &'b Handler);
    type Output = Result<Program<'a>>;

    fn do_pass((asg, handler): Self::Input) -> Self::Output {
        let pass = TransactionChecker {
            program: &asg,
            handler: handler,
            count: 0,
        };
        let mut director = VisitorDirector::new(pass);
        director.visit_program(&asg).ok();
        Ok(asg)
    }
}
