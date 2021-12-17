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

use leo_asg::{Annotation, Function};

use crate::AnnotationHandler;

pub(crate) struct CoreFunction {}

impl<'a> AnnotationHandler<'a> for CoreFunction {
    type Input = (&'a Function<'a>, &'a Annotation);
    type Output = ();

    fn resolve((function, annotation): Self::Input) -> Self::Output {
        function.core_mapping.replace(
            annotation
                .arguments
                .get(0)
                .or(Some(&function.name.borrow().name))
                .map(|f| f.to_string()),
        );
    }
}
