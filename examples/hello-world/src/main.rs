use brief::mail::Mailbox;

fn main() {
    let mailbox = Mailbox::try_from("name <user@domain.com>").unwrap();

    println!("{:#?}", mailbox);
}
