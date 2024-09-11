use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum GruPHstError {
    #[error("Attribute not found")]
    AttributeError 
}