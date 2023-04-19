/// Returns invalid_argument error for the first value that is not Some. Rebinds input symbols as non-optionals.
#[macro_export]
macro_rules! assert_is_some {
    ($($arg:ident),+) => {
        {
            let opts = [
            $(
                (std::stringify!($arg), $arg.is_some()),
            )+
            ];
            let missing_count = opts.iter().filter(|(_, is_some)| !is_some).count();
            if missing_count > 0 {
                let field_violations = opts.into_iter().filter_map(|(field_name, is_some)| {
                    if is_some {
                        return None;
                    }
                    Some(error::error_details::bad_request::FieldViolation {
                        field: Some(field_name.to_string()),
                        description: Some("missing".to_string()),
                    })
                }).collect::<Vec<_>>();
                let payload = error::error_details::BadRequest {
                    field_violations,
                };
                return Err(error::Error::invalid_argument_with("Missing required fields", Some(payload)).into());
            }
        }
        $(let $arg = $arg.unwrap();)+
    };
}

/// Returns an invalid argument error if the specified field on a struct is None. Otherwise, returns the field wrapped in
/// an `Ok()` Result
#[macro_export]
macro_rules! expect_field_is_some {
    ($struc:ident.$field:ident) => {{
        $struc.$field.ok_or_else(|| {
            error::Error::invalid_argument(format!(
                "Field {} on struct {} should be Some, but is None",
                stringify!($field),
                stringify!($struc)
            ))
        })
    }};
}

/// Generates an error which lists the field by name as invalid.
#[macro_export]
macro_rules! invalid_argument {
    ($message:expr, $invalid_arg:expr) => {{
        let violation = error::error_details::bad_request::FieldViolation {
            field: Some(std::stringify!($invalid_arg).to_string()),
            description: Some(format!("value={:?}", $invalid_arg)),
        };
        let payload = error::error_details::BadRequest { field_violations: vec![violation] };
        error::Error::invalid_argument_with($message, Some(payload))
    }};
}

/// Generates an error which lists the field by name as out of range.
#[macro_export]
macro_rules! out_of_range {
    ($message:expr, $invalid_arg:ident) => {{
        let violation = error::error_details::bad_request::FieldViolation {
            field: Some(std::stringify!($invalid_arg).to_string()),
            description: Some(format!("value={:?}", $invalid_arg)),
        };
        let payload = error::error_details::BadRequest { field_violations: vec![violation] };
        error::Error::out_of_range_with($message, Some(payload))
    }};
}
