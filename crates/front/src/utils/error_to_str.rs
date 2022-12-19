use validator::{ValidationError, ValidationErrors};

pub fn validErr_to_str(errs: &ValidationErrors) -> String {
    let mut res = "Unknow error".to_owned();

    if let Some(err) = errs.field_errors().get("email") {
        res = "Bad email".to_string();
    } else if let Some(err) = errs.field_errors().get("password") {
        res = "Password min length 8".to_string();
    }

    res
}