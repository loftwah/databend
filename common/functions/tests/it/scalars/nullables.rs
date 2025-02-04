// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_datavalues::prelude::*;
use common_exception::Result;
use common_functions::scalars::*;

use crate::scalars::scalar_function_test::test_scalar_functions;
use crate::scalars::scalar_function_test::ScalarFunctionTest;

#[test]
fn test_is_null_function() -> Result<()> {
    let tests = vec![ScalarFunctionTest {
        name: "is-null-passed",
        nullable: false,
        columns: vec![Series::new(vec![Some(1i32), Some(0i32), None]).into()],
        expect: Series::new(vec![false, false, true]).into(),
        error: "",
    }];

    test_scalar_functions(IsNullFunction::try_create_func("")?, &tests)
}

#[test]
fn test_is_not_null_function() -> Result<()> {
    let tests = vec![ScalarFunctionTest {
        name: "is-not-null-passed",
        nullable: false,
        columns: vec![Series::new(vec![Some(1i32), Some(0i32), None]).into()],
        expect: Series::new(vec![true, true, false]).into(),
        error: "",
    }];

    test_scalar_functions(IsNotNullFunction::try_create_func("")?, &tests)
}
