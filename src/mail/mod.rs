mod address;
mod header;
mod mail;
mod mailbox;
mod validate;

#[derive(Debug)]
pub enum InvalidPartError {
    IsEmpty,
    ContainsForbiddenCharacter(char),
    ContainsNonAsciiCharacter(char),
}

pub use address::Address;
pub use header::Header;
pub use mail::{Mail, MailBuilder};
pub use mailbox::Mailbox;
pub use validate::validate_part;
