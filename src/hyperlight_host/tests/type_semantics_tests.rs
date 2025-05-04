/*
Copyright 2024 The Hyperlight Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

//! Tests focused on verifying the semantic preservation of types across the
//! guest-host boundary. Each test calls a corresponding "echo" function in
//! the simpleguest binary.

use hyperlight_host::func::{ParameterValue, ReturnType, ReturnValue};
#[cfg(target_os = "windows")]
use serial_test::serial; // using LoadLibrary requires serial tests

// Include common test helpers
pub mod common;
use crate::common::get_simpleguest_sandboxes;

// Helper macro for asserting float/double equality, handling NaN
macro_rules! assert_float_eq {
    ($a:expr, $b:expr) => {
        assert!(
            ($a.is_nan() && $b.is_nan()) || ($a == $b),
            "Floats not equal: {:?} != {:?}", $a, $b
        );
    };
}

#[test]
#[cfg_attr(target_os = "windows", serial)]
fn test_string_semantics() {
    let test_values = vec![
        "".to_string(),
        "hello".to_string(),
        "a".repeat(1024), // Test a moderately long string
        "你好世界".to_string(), // Test multi-byte characters
        " \t\n\r ".to_string(), // Test whitespace
    ];

    for mut sandbox in get_simpleguest_sandboxes(None).into_iter() {
        for value in &test_values {
            let res = sandbox.call_guest_function_by_name(
                "Echo", // Using the existing Echo function for strings
                ReturnType::String,
                Some(vec![ParameterValue::String(value.clone())]),
            );

            match res {
                Ok(ReturnValue::String(returned_value)) => {
                    assert_eq!(&returned_value, value, "String value mismatch");
                }
                Ok(other) => panic!("Expected ReturnValue::String, got {:?}", other),
                Err(e) => panic!("Guest call failed for value '{}': {:?}", value, e),
            }
        }
    }
}
#[test]
#[cfg_attr(target_os = "windows", serial)]
fn test_i32_semantics() {
    let test_values = vec![
        0i32,
        1i32,
        -1i32,
        i32::MAX,
        i32::MIN,
        12345,
        -54321,
    ];

    for mut sandbox in get_simpleguest_sandboxes(None).into_iter() {
        for value in &test_values {
            let res = sandbox.call_guest_function_by_name(
                "EchoI32",
                ReturnType::Int,
                Some(vec![ParameterValue::Int(*value)]),
            );

            match res {
                Ok(ReturnValue::Int(returned_value)) => {
                    assert_eq!(returned_value, *value, "i32 value mismatch");
                }
                Ok(other) => panic!("Expected ReturnValue::Int, got {:?}", other),
                Err(e) => panic!("Guest call failed for value '{}': {:?}", value, e),
            }
        }
    }
}
#[test]
#[cfg_attr(target_os = "windows", serial)]
fn test_u32_semantics() {
    let test_values = vec![
        0u32,
        1u32,
        u32::MAX,
        u32::MIN, // Which is 0
        12345,
        54321,
    ];

    for mut sandbox in get_simpleguest_sandboxes(None).into_iter() {
        for value in &test_values {
            let res = sandbox.call_guest_function_by_name(
                "EchoU32",
                ReturnType::UInt,
                Some(vec![ParameterValue::UInt(*value)]),
            );

            match res {
                Ok(ReturnValue::UInt(returned_value)) => {
                    assert_eq!(returned_value, *value, "u32 value mismatch");
                }
                Ok(other) => panic!("Expected ReturnValue::UInt, got {:?}", other),
                Err(e) => panic!("Guest call failed for value '{}': {:?}", value, e),
            }
        }
    }
}
#[test]
#[cfg_attr(target_os = "windows", serial)]
fn test_i64_semantics() {
    let test_values = vec![
        0i64,
        1i64,
        -1i64,
        i64::MAX,
        i64::MIN,
        123456789012345,
        -543210987654321,
    ];

    for mut sandbox in get_simpleguest_sandboxes(None).into_iter() {
        for value in &test_values {
            let res = sandbox.call_guest_function_by_name(
                "EchoI64",
                ReturnType::Long, // i64 maps to Long
                Some(vec![ParameterValue::Long(*value)]),
            );

            match res {
                Ok(ReturnValue::Long(returned_value)) => {
                    assert_eq!(returned_value, *value, "i64 value mismatch");
                }
                Ok(other) => panic!("Expected ReturnValue::Long, got {:?}", other),
                Err(e) => panic!("Guest call failed for value '{}': {:?}", value, e),
            }
        }
    }
}
#[test]
#[cfg_attr(target_os = "windows", serial)]
fn test_u64_semantics() {
    let test_values = vec![
        0u64,
        1u64,
        u64::MAX,
        u64::MIN, // Which is 0
        123456789012345,
        543210987654321,
    ];

    for mut sandbox in get_simpleguest_sandboxes(None).into_iter() {
        for value in &test_values {
            let res = sandbox.call_guest_function_by_name(
                "EchoU64",
                ReturnType::ULong, // u64 maps to ULong
                Some(vec![ParameterValue::ULong(*value)]),
            );

            match res {
                Ok(ReturnValue::ULong(returned_value)) => {
                    assert_eq!(returned_value, *value, "u64 value mismatch");
                }
                Ok(other) => panic!("Expected ReturnValue::ULong, got {:?}", other),
                Err(e) => panic!("Guest call failed for value '{}': {:?}", value, e),
            }
        }
    }
}
#[test]
#[cfg_attr(target_os = "windows", serial)]
fn test_bool_semantics() {
    let test_values = vec![true, false];

    for mut sandbox in get_simpleguest_sandboxes(None).into_iter() {
        for value in &test_values {
            let res = sandbox.call_guest_function_by_name(
                "EchoBool",
                ReturnType::Bool,
                Some(vec![ParameterValue::Bool(*value)]),
            );

            match res {
                Ok(ReturnValue::Bool(returned_value)) => {
                    assert_eq!(returned_value, *value, "bool value mismatch");
                }
                Ok(other) => panic!("Expected ReturnValue::Bool, got {:?}", other),
                Err(e) => panic!("Guest call failed for value '{}': {:?}", value, e),
            }
        }
    }
}
// These lines seem to be duplicated/incorrectly placed closing braces from previous tests.
// Removing them as the correct braces were added above.
#[test]
#[cfg_attr(target_os = "windows", serial)]
fn test_vec_u8_semantics() {
    let test_values: Vec<Vec<u8>> = vec![
        vec![],
        vec![0],
        vec![1, 2, 3, 4, 5],
        vec![255],
        vec![0, 0, 0],
        (0..100).collect(), // A slightly larger vector
    ];

    for mut sandbox in get_simpleguest_sandboxes(None).into_iter() {
        for value in &test_values {
            let res = sandbox.call_guest_function_by_name(
                "EchoVecBytes",
                ReturnType::VecBytes,
                Some(vec![ParameterValue::VecBytes(value.clone())]),
            );

            match res {
                Ok(ReturnValue::VecBytes(returned_value)) => {
                    assert_eq!(&returned_value, value, "Vec<u8> value mismatch");
                }
                Ok(other) => panic!("Expected ReturnValue::VecBytes, got {:?}", other),
                Err(e) => panic!("Guest call failed for value '{:?}': {:?}", value, e),
            }
        }
    }
}

#[test]
#[cfg_attr(target_os = "windows", serial)]
fn test_f32_semantics() {
    let test_values = vec![
        0.0f32,
        1.0f32,
        -1.0f32,
        f32::MAX,
        f32::MIN, // Smallest positive normal
        f32::MIN_POSITIVE,
        -f32::MAX,
        123.456f32,
        -987.654f32,
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
    ];

    for mut sandbox in get_simpleguest_sandboxes(None).into_iter() {
        for value in &test_values {
            let res = sandbox.call_guest_function_by_name(
                "EchoFloat",
                ReturnType::Float,
                Some(vec![ParameterValue::Float(*value)]),
            );

            match res {
                Ok(ReturnValue::Float(returned_value)) => {
                    assert_float_eq!(returned_value, *value); // Use macro for NaN comparison
                }
                Ok(other) => panic!("Expected ReturnValue::Float, got {:?}", other),
                Err(e) => panic!("Guest call failed for value '{}': {:?}", value, e),
            }
        }
    }
}

#[test]
#[cfg_attr(target_os = "windows", serial)]
fn test_f64_semantics() {
    let test_values = vec![
        0.0f64,
        1.0f64,
        -1.0f64,
        f64::MAX,
        f64::MIN, // Smallest positive normal
        f64::MIN_POSITIVE,
        -f64::MAX,
        123.456789012345f64,
        -987.654321098765f64,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ];

    for mut sandbox in get_simpleguest_sandboxes(None).into_iter() {
        for value in &test_values {
            let res = sandbox.call_guest_function_by_name(
                "EchoDouble",
                ReturnType::Double, // f64 maps to Double
                Some(vec![ParameterValue::Double(*value)]),
            );

            match res {
                Ok(ReturnValue::Double(returned_value)) => {
                    assert_float_eq!(returned_value, *value); // Use macro for NaN comparison
                }
                Ok(other) => panic!("Expected ReturnValue::Double, got {:?}", other),
                Err(e) => panic!("Guest call failed for value '{}': {:?}", value, e),
            }
        }
    }
}

#[test]
#[cfg_attr(target_os = "windows", serial)]
fn test_void_return_semantics() {
    for mut sandbox in get_simpleguest_sandboxes(None).into_iter() {
        let res = sandbox.call_guest_function_by_name(
            "ReturnVoid",
            ReturnType::Void,
            None, // No parameters
        );

        match res {
            Ok(ReturnValue::Void) => {
                // Success, do nothing
            }
            Ok(other) => panic!("Expected ReturnValue::Void, got {:?}", other),
            Err(e) => panic!("Guest call failed for ReturnVoid: {:?}", e),
        }
    }
}