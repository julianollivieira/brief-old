use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum ParseUserError {
    ContainsForiddenCharacters,
    ContainsNonAsciiCharacters,
}

#[derive(Debug)]
pub enum ParseDomainError {
    ContainsForiddenCharacters,
    ContainsNonAsciiCharacters,
}

#[derive(Debug)]
pub enum ParseAddressError {
    InvalidUser(ParseUserError),
    InvalidDomain(ParseDomainError),
    MissingUserOrDomainError,
}

#[derive(Debug)]
pub enum ParseNameError {
    ContainsForiddenCharacters,
    ContainsNonAsciiCharacters,
}

#[derive(Debug)]
pub enum ParseMailBoxError {
    InvalidName(ParseNameError),
    InvalidAddress(ParseAddressError),
    MissingAngleBrackets,
    MissingOpeningAngleBracket,
    MissingClosingAngleBracket,
    InvalidOrderOfAngleBrackets,
}

/// Represents the user part of an address
#[derive(Debug)]
pub struct User(String);

impl FromStr for User {
    type Err = ParseUserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let forbidden_characters = ['@', '<', '>'];

        if s.contains(&forbidden_characters) {
            return Err(ParseUserError::ContainsForiddenCharacters);
        }

        if !s.is_ascii() {
            return Err(ParseUserError::ContainsNonAsciiCharacters);
        }

        // TODO: Check if I forgot to check other possible invalid name cases

        Ok(Self(s.to_string()))
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Represents the domain part of an address
#[derive(Debug)]
pub struct Domain(String);

impl FromStr for Domain {
    type Err = ParseDomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let forbidden_characters = ['@', '<', '>'];

        if s.contains(&forbidden_characters) {
            return Err(ParseDomainError::ContainsForiddenCharacters);
        }

        if !s.is_ascii() {
            return Err(ParseDomainError::ContainsNonAsciiCharacters);
        }

        // TODO: Check if I forgot to check other possible invalid name cases

        Ok(Self(s.to_string()))
    }
}

impl Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Represents an address, which is a combination of a user and a domain
#[derive(Debug)]
pub struct Address {
    user: User,
    domain: Domain,
}

impl Address {
    /// Creates a new address from a user and domain, returning an error if the user and/or
    /// domain are invalid
    ///
    /// ```
    /// use brief::address::Address;
    ///
    /// let address = Address::try_new("user", "domain.com").unwrap();
    /// ````
    pub fn try_new(user: &str, domain: &str) -> Result<Self, ParseAddressError> {
        Ok(Self {
            user: user.parse().map_err(ParseAddressError::InvalidUser)?,
            domain: domain.parse().map_err(ParseAddressError::InvalidDomain)?,
        })
    }
}

impl FromStr for Address {
    type Err = ParseAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if !trimmed.contains('@') {
            return Err(ParseAddressError::MissingUserOrDomainError);
        }

        let split: Vec<&str> = s.rsplitn(2, '@').collect();
        Ok(Self {
            user: split[1].parse().map_err(ParseAddressError::InvalidUser)?,
            domain: split[0].parse().map_err(ParseAddressError::InvalidDomain)?,
        })
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}@{}", self.user, self.domain))
    }
}

/// Represents a name for a mail box
#[derive(Debug)]
pub struct Name(String);

impl FromStr for Name {
    type Err = ParseNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let forbidden_characters = ['@', '<', '>'];

        if s.contains(&forbidden_characters) {
            return Err(ParseNameError::ContainsForiddenCharacters);
        }

        if !s.is_ascii() {
            return Err(ParseNameError::ContainsNonAsciiCharacters);
        }

        // TODO: Check if I forgot to check other possible invalid name cases

        Ok(Self(s.to_string()))
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Represents a mail box, which is a combination of an address and possibly a name
#[derive(Debug)]
pub struct MailBox {
    pub name: Option<Name>,
    pub address: Address,
}

impl MailBox {
    /// Creates a new mailbox from a name and address, returning an error if the name is
    /// invalid
    ///
    /// ```
    /// use brief::address::{Address, MailBox};
    ///
    /// let address = Address::try_new("user", "domain.com").unwrap();
    /// let mail_box = MailBox::try_new(Some("name"), address).unwrap();
    /// ````
    pub fn try_new(name: Option<&str>, address: Address) -> Result<Self, ParseMailBoxError> {
        Ok(Self {
            name: name
                .map(|n| n.parse().map_err(ParseMailBoxError::InvalidName))
                .transpose()?,
            address,
        })
    }
}

impl FromStr for MailBox {
    type Err = ParseMailBoxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        match (trimmed.find('<'), trimmed.find('>')) {
            (None, None) => Err(ParseMailBoxError::MissingAngleBrackets),
            (None, Some(_)) => Err(ParseMailBoxError::MissingOpeningAngleBracket),
            (Some(_), None) => Err(ParseMailBoxError::MissingClosingAngleBracket),
            (Some(left), Some(right)) => {
                if left > right {
                    return Err(ParseMailBoxError::InvalidOrderOfAngleBrackets);
                }

                let split = trimmed.split_once('<').unwrap();

                let mut name: Option<String> = None;
                if !split.0.is_empty() {
                    name = Some(split.0.trim().to_string());
                }

                let address = split.1.split_once('>').unwrap().0;

                return Ok(Self {
                    name: name
                        .map(|n| n.parse().map_err(ParseMailBoxError::InvalidName))
                        .transpose()?,
                    address: address.parse().map_err(ParseMailBoxError::InvalidAddress)?,
                });
            }
        }
    }
}

impl Display for MailBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            f.write_fmt(format_args!("{} <{}>", name, self.address))
        } else {
            f.write_fmt(format_args!("{}", self.address))
        }
    }
}

// TODO: extensive testing

