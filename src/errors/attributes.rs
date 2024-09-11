use std::{error::Error, fmt};

#[derive(PartialEq)]
pub struct AttributeError;

impl fmt::Display for AttributeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Attribute not found")
    }
}

impl fmt::Debug for AttributeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl Error for AttributeError {}