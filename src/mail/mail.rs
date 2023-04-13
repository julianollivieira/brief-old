use super::Header;

pub struct MailData<'a> {
    headers: &'a [Header],
    data: &'a [u8],
}

pub struct Mail<'a> {
    data: MailData<'a>,
}

pub struct MailBuilder {
    //
}

impl MailBuilder {
    pub fn new() -> Self {
        Self {}
    }
}
