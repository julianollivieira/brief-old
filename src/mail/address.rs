use super::{validate_part, InvalidPartError};

#[derive(Debug)]
pub enum ParseAddressError {
    MissingUserOrDomain,
    InvalidUser(InvalidPartError),
    InvalidDomain(InvalidPartError),
}

/// Represents an email address
///
/// You can create an `Address` from a user string and domain string:
/// ```
/// use brief::mail::Address;
///
/// let address = Address::try_new("user", "domain.com").unwrap();
/// ```
///
/// or from a string:
/// ```
/// use brief::mail::Address;
///
/// let address = Address::try_from("user@domain.com").unwrap();
/// ```
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Address<'a> {
    user: &'a str,
    domain: &'a str,
}

impl<'a> Address<'a> {
    /// Tries to create an address from a user and domain, returning an error if the user and/or
    /// domain are invalid.
    ///
    /// ```
    /// use brief::mail::Address;
    ///
    /// let address = Address::try_new("user", "domain.com").unwrap();
    /// ```
    pub fn try_new(user: &'a str, domain: &'a str) -> Result<Self, ParseAddressError> {
        validate_part(user).map_err(|e| ParseAddressError::InvalidUser(e))?;
        validate_part(domain).map_err(|e| ParseAddressError::InvalidDomain(e))?;

        // TODO: can't throw 'MissingUserOrDomain' but TryFrom impl can

        Ok(Self { user, domain })
    }
}

impl<'a> TryFrom<&'a str> for Address<'a> {
    type Error = ParseAddressError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if !value.contains('@') {
            return Err(ParseAddressError::MissingUserOrDomain);
        }

        let mut split = value.rsplitn(2, '@');
        let domain = split.next().unwrap_or("");
        let user = split.next().unwrap_or("");

        validate_part(user).map_err(|e| ParseAddressError::InvalidUser(e))?;
        validate_part(domain).map_err(|e| ParseAddressError::InvalidDomain(e))?;

        Address::try_new(user, domain)
    }
}

#[cfg(test)]
mod tests {
    use super::Address;

    #[test]
    fn it_creates_an_address() {
        let address = Address::try_new("user", "domain.com");
        assert!(address.is_ok());
    }

    #[test]
    fn it_fails_to_create_an_address_when_user_or_domain_is_empty() {
        let without_user = Address::try_new("", "domain.com");
        assert!(without_user.is_err());

        let without_domain = Address::try_new("name", "");
        assert!(without_domain.is_err());
    }

    #[test]
    fn it_creates_an_address_from_a_string_with_a_valid_user_and_domain() {
        let address = Address::try_from("user@domain.com");
        assert!(address.is_ok());
    }

    #[test]
    fn it_fails_to_create_an_address_from_a_string_without_user_or_domain_is_empty() {
        let without_user = Address::try_from("@domain.com");
        assert!(without_user.is_err());

        let without_domain = Address::try_from("name");
        assert!(without_domain.is_err());
    }
}
