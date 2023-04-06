use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum ParseAddressError {
    MissingAtSymbolError,
}

impl Display for ParseAddressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ParseAddressError::MissingAtSymbolError => {
                "Missing domain or user (no '@' symbol found)"
            }
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
            return Err(ParseAddressError::MissingAtSymbolError);
        }

        let parts: Vec<&str> = s.split('@').collect();
        let user = parts[0].to_string();
        let domain = parts[1].to_string();

        // TODO: check if user and domain are actually valid (according to spec)

        Ok(Self { user, domain })
    }
}

#[derive(Debug)]
pub struct MailBox {
    pub name: String,
    pub address: Address,
}

impl Display for MailBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} <{}>", self.name, self.address))
    }
}

impl FromStr for MailBox {
    // TODO: ParseMailBoxError
    type Err = ParseAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }

    // TODO: add tests
}

#[cfg(test)]
mod test {
    use crate::address::ParseAddressError;

    use super::Address;

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
}
