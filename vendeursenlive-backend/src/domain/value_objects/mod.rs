use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ValueObjectError {
    #[error("phone number cannot be empty")]
    EmptyPhoneNumber,
    #[error("phone number must use a valid international format")]
    InvalidPhoneNumber,
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
    #[error("TikTok live URL must identify a public LIVE on www.tiktok.com")]
    InvalidTikTokLiveUrl,
    #[error("TikTok username is invalid")]
    InvalidTikTokUsername,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(value: impl Into<String>) -> Result<Self, ValueObjectError> {
        let value = value.into();
        let value = value.trim();

        if value.is_empty() {
            return Err(ValueObjectError::EmptyPhoneNumber);
        }

        let mut digits: String = value.chars().filter(char::is_ascii_digit).collect();
        if digits.starts_with("00") {
            digits.drain(..2);
        } else if digits.len() == 10 && digits.starts_with('0') {
            digits = format!("225{digits}");
        }

        if !(8..=15).contains(&digits.len()) {
            return Err(ValueObjectError::InvalidPhoneNumber);
        }

        Ok(Self(format!("+{digits}")))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod phone_number_tests {
    use super::*;

    #[test]
    fn normalizes_an_ivorian_local_number() {
        let phone = PhoneNumber::new("07 00 00 00 00").expect("valid phone");
        assert_eq!(phone.as_str(), "+2250700000000");
    }

    #[test]
    fn preserves_an_international_number() {
        let phone = PhoneNumber::new("+225 07 00 00 00 00").expect("valid phone");
        assert_eq!(phone.as_str(), "+2250700000000");
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
pub struct TikTokLiveUrl {
    canonical_url: String,
    username: String,
}

impl TikTokLiveUrl {
    pub fn new(value: impl Into<String>) -> Result<Self, ValueObjectError> {
        let value = value.into().trim().to_owned();

        if value.is_empty() {
            return Err(ValueObjectError::EmptyTikTokLiveUrl);
        }

        let username = value
            .strip_prefix("https://www.tiktok.com/@")
            .and_then(|path| path.strip_suffix("/live"))
            .ok_or(ValueObjectError::InvalidTikTokLiveUrl)?;

        let live_url = Self::from_username(username)?;
        if live_url.canonical_url != value {
            return Err(ValueObjectError::InvalidTikTokLiveUrl);
        }

        Ok(live_url)
    }

    pub fn from_username(username: impl Into<String>) -> Result<Self, ValueObjectError> {
        let username = username.into();
        let username = username.trim().trim_start_matches('@');

        if username.is_empty()
            || username.len() > 64
            || !username.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '_' | '.')
            })
        {
            return Err(ValueObjectError::InvalidTikTokUsername);
        }

        Ok(Self {
            canonical_url: format!("https://www.tiktok.com/@{username}/live"),
            username: username.to_owned(),
        })
    }

    pub fn as_str(&self) -> &str {
        &self.canonical_url
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}

#[cfg(test)]
mod tiktok_live_url_tests {
    use super::*;

    #[test]
    fn builds_a_canonical_live_url_from_a_username() {
        let live_url = TikTokLiveUrl::from_username("@vendeursenlive").expect("valid username");

        assert_eq!(
            live_url.as_str(),
            "https://www.tiktok.com/@vendeursenlive/live"
        );
        assert_eq!(live_url.username(), "vendeursenlive");
    }

    #[test]
    fn rejects_non_canonical_or_untrusted_urls() {
        assert!(TikTokLiveUrl::new("https://tiktok.com/@seller/live").is_err());
        assert!(TikTokLiveUrl::new("https://www.tiktok.com.evil.test/@seller/live").is_err());
        assert!(TikTokLiveUrl::new("https://www.tiktok.com/@seller/video/123").is_err());
    }
}
