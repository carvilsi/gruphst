use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum GruPHstError {
    #[error("Attribute not found")]
    AttributeError,
    #[error("Vertex not found")]
    VertexError,
    #[error("Unknown GruPHst Error")]
    Unknown
}