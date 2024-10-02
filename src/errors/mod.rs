use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum GruPHstError {
    #[error("Attribute not found")]
    AttributeNotFound,
    #[error("Attributes empty")]
    AttributesEmpty,
    #[error("Vertex not found")]
    VertexNotFound,
    #[error("Edge not found")]
    EdgeNotFound,
    #[error("No relations: \"{0}\" on Edges")]
    EdgeNoRelations(String),
    #[error("Vault is empy; no Edges")]
    VaultEmpty,
    #[error("Provided vault: \"{0}\" does not exists")]
    VaultNotExists(String),
    #[error("No vaults on Grpahs")]
    NoVaultOnGraphs,
    #[error("Persisted file excedes max memory usage, check GRUPHST_MAX_MEM_USAGE var")]
    PersistenceFile,
    #[error("Not valid file on path")]
    NotValidFileNameOnPath,
    #[error("CSV file empty")]
    CSVEmpty,
    #[error("Unknown GruPHst Error")]
    Unknown
}