use derive_new::new;
use serde::{Serialize, Deserialize};
use serde_with::{serde_as, DisplayFromStr};
use crate::domain::model::error::DomainError;
use crate::domain::model::result::ValidationResult;
use crate::domain::model::validation::Validator;

#[derive(Debug, new, PartialEq, Clone)]
pub struct  UserID {
    value: String,
}

impl std::str::FromStr for UserID {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}

impl std::fmt::Display for UserID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.value.as_str())
    }
}

impl Validator for UserID {
    fn validate(&self) -> ValidationResult {
        if self.value.len() != 10 {
            return Err(DomainError::ValidationError("UserID".to_string(), "invalid format".to_string()));
        }
        Ok(())
    }
}

impl UserID {
    pub fn generate() -> Self {
        Self::new(ramdomstr::alphabet(10))
    }
}

#[derive(Debug, new, Clone)]
pub struct UserName {
    value: String,
}

impl std::str::FromStr for UserName {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}

impl std::fmt::Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.value.as_str())
    }
}

impl Validator for UserName {
    fn validate(&self) -> ValidationResult {
        if self.value.len() < 2 {
            return Err(DomainError::ValidationError("UserName".to_string(), "too short".to_string()));
        }
        if self.value.len() > 10 {
            return Err(DomainError::ValidationError("UserName".to_string(), "too long".to_string()));
        }
        Ok(())
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, new)]
pub struct User {
    #[serde_as(as = "DisplayFromStr")]
    pub id: UserID,

    #[serde_as(as = "DisplayFromStr")]
    pub name: UserName,
}

impl User {
    pub fn generate(name: UserName) -> Self {
        Self {
            id: UserID::generate(),
            name,
        }
    }
}

impl Validator for User {
    fn validate(&self) -> ValidationResult {
        self.id.validate()?;
        self.name.validate()?;
        Ok(())
    }
}
