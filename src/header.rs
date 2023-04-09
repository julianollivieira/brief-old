use crate::address::MailBox;

/// All Common Internet Message Headers, implemented according to RFC 2076
///
/// See: <https://www.rfc-editor.org/rfc/rfc2076>
pub enum Header<'a> {
    /// Represents the 'Return-Path' header
    ///
    /// Used to convey the information from the MAIL FROM envelope attribute in final delivery, when
    /// the message leaves the SMTP environment in which "MAIL FROM" is used.
    ReturnPath(MailBox<'a>),
}

impl Header<'_> {
    pub fn name(&self) -> String {
        String::from(match self {
            Header::ReturnPath(_) => "Return-Path",
        })
    }
    pub fn body(&self) -> String {
        match self {
            // TODO: convert email address to string
            Header::ReturnPath(_) => format!("<{}>", "email@addres.com"),
        }
    }
}

impl ToString for Header<'_> {
    fn to_string(&self) -> String {
        format!("{}: {}", self.name(), self.body())
    }
}
