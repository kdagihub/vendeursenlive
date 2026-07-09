use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ValueObjectError {
    #[error("phone number cannot be empty")]
    EmptyPhoneNumber,
    #[error("email address cannot be empty")]
    EmptyEmail,
    #[error("email address is invalid")]
    InvalidEmail,
    #[error("price must be greater than zero")]
    InvalidPrice,
    #[error("customer name cannot be empty")]
    EmptyCustomerName,
    #[error("customer location cannot be empty")]
    EmptyCustomerLocation,
    #[error("TikTok live URL cannot be empty")]
    EmptyTikTokLiveUrl,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(value: impl Into<String>) -> Result<Self, ValueObjectError> {
        let value = value.into().trim().to_owned();

        if value.is_empty() {
            return Err(ValueObjectError::EmptyPhoneNumber);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new(value: impl Into<String>) -> Result<Self, ValueObjectError> {
        let value = value.into().trim().to_ascii_lowercase();

        if value.is_empty() {
            return Err(ValueObjectError::EmptyEmail);
        }

        if !value.contains('@') {
            return Err(ValueObjectError::InvalidEmail);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price(i32);

impl Price {
    pub fn from_fcfa(amount: i32) -> Result<Self, ValueObjectError> {
        if amount <= 0 {
            return Err(ValueObjectError::InvalidPrice);
        }

        Ok(Self(amount))
    }

    pub fn amount_fcfa(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomerContact {
    pub name: String,
    pub phone_number: PhoneNumber,
    pub location: String,
}

impl CustomerContact {
    pub fn new(
        name: impl Into<String>,
        phone_number: PhoneNumber,
        location: impl Into<String>,
    ) -> Result<Self, ValueObjectError> {
        let name = name.into().trim().to_owned();
        let location = location.into().trim().to_owned();

        if name.is_empty() {
            return Err(ValueObjectError::EmptyCustomerName);
        }

        if location.is_empty() {
            return Err(ValueObjectError::EmptyCustomerLocation);
        }

        Ok(Self {
            name,
            phone_number,
            location,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TikTokLiveUrl(String);

impl TikTokLiveUrl {
    pub fn new(value: impl Into<String>) -> Result<Self, ValueObjectError> {
        let value = value.into().trim().to_owned();

        if value.is_empty() {
            return Err(ValueObjectError::EmptyTikTokLiveUrl);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
