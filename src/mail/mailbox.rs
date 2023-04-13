use super::{address::ParseAddressError, validate_part, Address, InvalidPartError};

#[derive(Debug)]
pub enum ParseMailboxError {
    MissingAngleBrackets,
    MissingOpeningAngleBracket,
    MissingClosingAngleBracket,
    WrongOrderAngleBrackets,
    InvalidName(InvalidPartError),
    InvalidAddress(ParseAddressError),
}

impl From<ParseAddressError> for ParseMailboxError {
    fn from(value: ParseAddressError) -> Self {
        Self::InvalidAddress(value)
    }
}

/// Represents a mailbox
///
/// You can create a `Mailbox` from a name string and an addres:
/// ```
/// use brief::mail::Mailbox;
///
/// let mailbox = Mailbox::try_new(Some("name"), "user@domain.com".try_into().unwrap()).unwrap();
/// ```
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Mailbox<'a> {
    pub name: Option<&'a str>,
    pub address: Address<'a>,
}

impl<'a> Mailbox<'a> {
    /// Tries to create a mailbox from a name and address, returning an error if the name is
    /// invalid.
    ///
    /// ```
    /// use brief::mail::Mailbox;
    ///
    /// let mailbox = Mailbox::try_new(Some("name"), "user@domain.com".try_into().unwrap()).unwrap();
    /// ```
    pub fn try_new(name: Option<&'a str>, address: Address<'a>) -> Result<Self, ParseMailboxError> {
        if let Some(name) = name {
            validate_part(name).map_err(|e| ParseMailboxError::InvalidName(e))?;
        }

        // TODO: can't throw 'InvalidAddress' but TryFrom impl can

        Ok(Self { name, address })
    }
}

impl<'a> TryFrom<&'a str> for Mailbox<'a> {
    type Error = ParseMailboxError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match (value.find('<'), value.find('>')) {
            (None, None) => Err(ParseMailboxError::MissingAngleBrackets),
            (None, Some(_)) => Err(ParseMailboxError::MissingOpeningAngleBracket),
            (Some(_), None) => Err(ParseMailboxError::MissingClosingAngleBracket),
            (Some(left), Some(right)) => {
                if left > right {
                    return Err(ParseMailboxError::WrongOrderAngleBrackets);
                }

                // we can unwrap here because we are sure the string includes a '<'.
                let (name_str, rest) = value.split_once('<').unwrap();
                let address_str = rest.split_once('>').unwrap().0;

                let name = (!name_str.is_empty()).then(|| name_str).or_else(|| None);
                let address = Address::try_from(address_str)?;

                Ok(Self { name, address })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mail::Mailbox;

    #[test]
    fn it_creates_a_mailbox() {
        let mailbox = Mailbox::try_new(Some("name"), "user@domain.com".try_into().unwrap());
        assert!(mailbox.is_ok());
    }

    #[test]
    fn it_creates_a_mailbox_when_name_is_empty() {
        let mailbox = Mailbox::try_new(None, "user@domain.com".try_into().unwrap());
        assert!(mailbox.is_ok());
    }

    #[test]
    fn it_creates_a_mailbox_from_a_string_with_a_valid_name_and_address() {
        let mailbox = Mailbox::try_from("name <user@domain.com>");
        assert!(mailbox.is_ok());
    }

    #[test]
    fn it_creates_a_mailbox_from_a_string_with_an_address() {
        let mailbox = Mailbox::try_from("<user@domain.com>");
        assert!(mailbox.is_ok());
    }

    #[test]
    fn it_fails_when_the_brackets_are_invali() {
        let cases = [
            Mailbox::try_from("user user@domain.com>").is_err(),
            Mailbox::try_from("user <user@domain.com").is_err(),
            Mailbox::try_from("user user@domain.com").is_err(),
            Mailbox::try_from("user >user@domain.com<").is_err(),
            Mailbox::try_from("user@domain.com>").is_err(),
            Mailbox::try_from("<user@domain.com").is_err(),
            Mailbox::try_from("user@domain.com").is_err(),
            Mailbox::try_from(">user@domain.com<").is_err(),
        ];

        assert!(cases.iter().any(|c| *c))
    }
}
