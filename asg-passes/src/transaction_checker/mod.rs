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

use leo_asg::*;
use leo_errors::{emitter::Handler, CompilerError, LeoError, Result, Span};

use snarkvm_dpc::{network::testnet2::Testnet2, Network};

/// Checks that functions with @transition or @transaction annotations are well formed.
pub struct TransactionChecker<'b> {
    handler: &'b Handler,
    count: u8,
}

impl<'a, 'b> TransactionChecker<'b> {
    /// Checks that a given type is a `Circuit` and a `Record` from stdlib,
    /// otherwise emit an error.
    fn check_type_is_record(&self, typ: &Type<'a>, err: &dyn Fn() -> LeoError) {
        match typ {
            Type::Circuit(circ) => {
                if circ.name.clone().into_inner().to_string() != "Record"
                    || !circ.annotations.keys().any(|k| k == "CoreCircuit")
                {
                    self.handler.emit_err(err())
                }
            }
            _ => self.handler.emit_err(err()),
        }
    }

    /// Checks that argument types are `Record`s and that there are an appropriate nubmer of them.
    fn check_arg_types(&self, typ: &Type<'a>, max_records: usize, err: &dyn Fn() -> LeoError) {
        match typ {
            Type::Circuit(_) => self.check_type_is_record(typ, err),
            Type::Tuple(sub_typs) => {
                if sub_typs.len() > max_records {
                    self.handler.emit_err(err());
                }
                sub_typs.iter().for_each(|typ| self.check_type_is_record(typ, err));
            }
            _ => self.handler.emit_err(err()),
        }
    }
}

impl<'a, 'b> ExpressionVisitor<'a> for TransactionChecker<'b> {}

impl<'a, 'b> StatementVisitor<'a> for TransactionChecker<'b> {}

impl<'a, 'b> ProgramVisitor<'a> for TransactionChecker<'b> {
    fn visit_function(&mut self, input: &'a Function<'a>) -> VisitResult {
        // Temporary requirement restricting transactions to transitions
        if input.annotations.keys().any(|k| k == &"transaction".to_string()) {
            if !input.annotations.keys().any(|k| &"transition".to_string()) {
                unimplemented!("Standalone transactions have not been implemented. Each @transaction must contain @transition.")
            }
        }

        if input.annotations.keys().any(|k| k == &"transition".to_string()) {
            // Temporary requirement restricting transitions to transactions
            if !input.annotations.keys().any(|k| k == &"transaction".to_string()) {
                unimplemented!("Standalone transitions have not been implemented. Each @transition must contain @transaction.")
            }

            let default = Span::default();
            let span = &input.span.as_ref().unwrap_or(&default);

            // Cannot have more than allowed number of transitions for a given program
            if self.count == Testnet2::NUM_TRANSITIONS {
                self.handler.emit_err(
                    CompilerError::exceeded_maximum_number_of_transitions(Testnet2::NUM_TRANSITIONS, span).into(),
                );
            }

            // Check that function arguments have the appropriate types.
            let err = || CompilerError::input_is_at_most_n_records(Testnet2::NUM_INPUT_RECORDS, span).into();
            let arg_types: Type<'a> = if input.arguments.len() == 1 {
                let (_, val) = input.arguments.last().unwrap();
                val.get().borrow().type_.clone()
            } else {
                Type::Tuple(
                    input
                        .arguments
                        .values()
                        .map(|v| v.get().borrow().type_.clone())
                        .collect(),
                )
            };
            self.check_arg_types(&arg_types, Testnet2::NUM_INPUT_RECORDS, &err);

            // Check that function outputs have the appropriate types.
            let err = || CompilerError::output_is_at_most_n_records(Testnet2::NUM_OUTPUT_RECORDS, span).into();
            self.check_arg_types(&input.output, Testnet2::NUM_OUTPUT_RECORDS, &err);
            self.count += 1;
        }

        VisitResult::SkipChildren
    }
}

impl<'a, 'b> AsgPass<'a> for TransactionChecker<'b> {
    type Input = (Program<'a>, &'b Handler);
    type Output = Result<Program<'a>>;

    fn do_pass((asg, handler): Self::Input) -> Self::Output {
        let pass = TransactionChecker { handler, count: 0 };
        let mut director = VisitorDirector::new(pass);
        director.visit_program(&asg).ok();
        Ok(asg)
    }
}
