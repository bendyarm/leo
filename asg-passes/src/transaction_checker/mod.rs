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

impl<'a, 'b> ExpressionVisitor<'a> for TransactionChecker<'b> {}

impl<'a, 'b> StatementVisitor<'a> for TransactionChecker<'b> {}

impl<'a, 'b> ProgramVisitor<'a> for TransactionChecker<'b> {
    fn visit_function(&mut self, input: &'a Function<'a>) -> VisitResult {
        if input.annotations.keys().any(|k| k == &"transition".to_string()) {
            let default = Span::default();
            let span = &input.span.as_ref().unwrap_or(&default);

            // Cannot have more than allowed number of transitions for a given program
            if self.count == Testnet2::NUM_TRANSITIONS {
                self.handler.emit_err(
                    CompilerError::exceeded_maximum_number_of_transitions(Testnet2::NUM_TRANSITIONS, span).into(),
                );
            }

            let handle_non_record_circuit = |cir1: &Circuit, cir2: Option<&Circuit>, err: LeoError| {
                let mut is_stdlib_record = cir1.name.clone().into_inner().name.to_string().eq("Record")
                    && cir1.annotations.keys().any(|k| k == &"CoreCircuit");
                if let Some(cir) = cir2 {
                    is_stdlib_record &= cir.name.clone().into_inner().name.to_string().eq("Record")
                        && cir.annotations.keys().any(|k| k == &"CoreCircuit");
                }
                if !is_stdlib_record {
                    self.handler.emit_err(err)
                }
            };

            let check_arg_types = |arg_typs: &[Type<'a>], err: LeoError| match arg_typs[..] {
                [] => (),
                [Type::Circuit(cir)] => handle_non_record_circuit(cir, None, err),
                [Type::Circuit(cir1), Type::Circuit(cir2)] => {
                    handle_non_record_circuit(cir1, Some(cir2), err);
                }
                _ => self.handler.emit_err(err),
            };

            // Check that function arguments have the appropriate types.
            let err: LeoError = CompilerError::output_is_at_most_n_records(Testnet2::NUM_OUTPUT_RECORDS, span).into();
            let arg_typs: Vec<Type<'a>> = input
                .arguments
                .values()
                .map(|v| v.get().borrow().type_.clone())
                .collect();
            check_arg_types(&arg_typs[..], err);

            // Check that function outputs have the appropriate types.
            let err: LeoError = CompilerError::input_is_at_most_n_records(Testnet2::NUM_INPUT_RECORDS, span).into();
            match &input.output {
                Type::Tuple(arg_typs) => check_arg_types(&arg_typs[..], err),
                Type::Circuit(cir) => handle_non_record_circuit(cir, None, err),
                _ => self.handler.emit_err(err),
            }
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
