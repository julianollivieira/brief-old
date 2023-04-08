use brief::address::MailBox;

fn main() {
    let mail_box: MailBox = "Name <user@domain>".parse().unwrap();

    println!("{:#?}", mail_box);
}
