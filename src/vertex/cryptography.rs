use std::error::Error;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use super::Vertex;

impl Vertex {
    /// Adds an attribute str cryptographic hashed with Argon2
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut vertex = Vertex::new("Brian");
    /// vertex.set_hash("password", "53cr37");
    /// ```
    pub fn set_hash(&mut self, attr_hash_key: &str, plain_text: &str) {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let argon2_hash = argon2
            .hash_password(plain_text.as_bytes(), &salt)
            .unwrap()
            .to_string();
        self.set_attr(attr_hash_key, argon2_hash);
    }

    /// Checks if an attribute str cryptographic hashed with Argon2
    /// matches with the plain text
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut vertex = Vertex::new("Brian");
    /// vertex.set_hash("password", "53cr37");
    /// assert!(vertex.is_hash_valid("password", "53cr37").unwrap());
    /// assert!(!vertex.is_hash_valid("password", "f00b4r").unwrap());
    /// ```
    pub fn is_hash_valid(
        &self,
        attr_hash_key: &str,
        plain_text: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let hash_value = self.get_attr(attr_hash_key)?;
        let parsed_hash = PasswordHash::new(hash_value.as_str()).unwrap();
        Ok(Argon2::default()
            .verify_password(plain_text.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
