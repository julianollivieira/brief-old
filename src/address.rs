use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum ParseAddressError {
    MissingUserOrDomainError,
}

#[derive(Debug)]
pub enum ParseMailBoxError {
    MissingAngleBracketsError,
    AngleMismatchError,
}

impl Error for ParseAddressError {}
impl Error for ParseMailBoxError {}

impl Display for ParseAddressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ParseAddressError::MissingUserOrDomainError => {
                "Missing user or domain (no '@' symbol found)"
            }
        })
    }
}

impl Display for ParseMailBoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ParseMailBoxError::MissingAngleBracketsError => "Missing angle bracket(s)",
            ParseMailBoxError::AngleMismatchError => "Angle bracket(s) mismatch",
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Address {
    pub user: String,
    pub domain: String,
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}@{}", self.user, self.domain))
    }
}

impl FromStr for Address {
    type Err = ParseAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains('@') {
            return Err(ParseAddressError::MissingUserOrDomainError);
        }

        let parts: Vec<&str> = s.split('@').collect();
        let user = parts[0].to_string();
        let domain = parts[1].to_string();

        // TODO: check if user and domain are actually valid (according to spec)

        Ok(Self { user, domain })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MailBox {
    pub name: Option<String>,
    pub address: Address,
}

impl Display for MailBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} <{}>",
            self.name.clone().unwrap_or(" ".to_string()),
            self.address
        ))
    }
}

// TODO: refactor
impl FromStr for MailBox {
    type Err = ParseMailBoxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: check if US-ASCII only

        let left_angle_occurrences = s.matches('<').count();
        let right_angle_occurrences = s.matches('>').count();

        if left_angle_occurrences > 1 || right_angle_occurrences > 1 {
            return Err(ParseMailBoxError::AngleMismatchError);
        }

        // we can use find because US-ASCII only according to spec
        let left_angle_bracket = s.find('<');
        let right_angle_bracket = s.find('>');

        if let (Some(left), Some(right)) = (left_angle_bracket, right_angle_bracket) {
            if left > right {
                return Err(ParseMailBoxError::AngleMismatchError);
            } else {
                let trimmed = s.trim();
                let mut name: Option<String> = None;

                if left != 0 {
                    name = Some(trimmed.split('<').collect::<Vec<&str>>()[0].to_string());
                }

                let a: Vec<&str> = trimmed.split('<').collect();
                let b: Vec<&str> = a[1].split('>').collect();
                let address = b[0];

                return Ok(MailBox {
                    address: address.parse().unwrap(),
                    name: name.map(|e| e.trim().to_string()),
                });
            }
        } else {
            return Err(ParseMailBoxError::MissingAngleBracketsError);
        }
    }
}

#[derive(Debug)]
pub struct MailBoxes(pub Vec<MailBox>);

impl Display for MailBoxes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .0
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

impl FromStr for MailBoxes {
    type Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::address::{Address, MailBox, ParseAddressError, ParseMailBoxError};

    #[test]
    fn it_formats_an_address_correctly() {
        let address = Address {
            user: String::from("user"),
            domain: String::from("example.com"),
        };

        assert_eq!(address.to_string(), "user@example.com");
    }

    #[test]
    fn it_succesfully_parses_a_correct_address() {
        let expected_address = Address {
            user: String::from("user"),
            domain: String::from("example.com"),
        };
        let actual_address: Result<Address, ParseAddressError> = "user@example.com".parse();

        assert!(actual_address.is_ok());
        assert_eq!(actual_address.unwrap(), expected_address);
    }

    #[test]
    fn it_fails_to_parse_an_incorrect_address() {
        let actual_result: Result<Address, ParseAddressError> = "userexample.com".parse();

        assert!(actual_result.is_err())
    }

    #[test]
    fn it_formats_a_mailbox_correctly() {
        let mailbox = MailBox {
            name: Some(String::from("My Name")),
            address: Address {
                user: String::from("user"),
                domain: String::from("example.com"),
            },
        };

        assert_eq!(mailbox.to_string(), "My Name <user@example.com>");
    }

    #[test]
    fn it_succesfully_parses_a_correct_mailbox() {
        let expected_mailbox = MailBox {
            name: Some(String::from("My Name")),
            address: Address {
                user: String::from("user"),
                domain: String::from("example.com"),
            },
        };

        let actual_mailbox: Result<MailBox, ParseMailBoxError> =
            "My Name <user@example.com>".parse();

        assert!(actual_mailbox.is_ok());
        assert_eq!(actual_mailbox.unwrap(), expected_mailbox);
    }
}
