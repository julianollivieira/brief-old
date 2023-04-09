use brief::address::MailBox;

fn main() {
    let mail_box = MailBox::try_from("Name <user@domain>").unwrap();

    println!("{:#?}", mail_box);
}
