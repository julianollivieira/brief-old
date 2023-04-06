use brief::address::{Address, MailBox};

fn main() {
    let address = Address {
        user: String::from("user"),
        domain: String::from("example.com"),
    };

    println!("address: {}", address);

    let mail_box = MailBox {
        name: String::from("John"),
        address,
    };

    println!("mail box: {}", mail_box);
    // let message = message_builder()
    //     .from("sender@example.com".parse().unwrap())
    //     .to("recipient@example.com".parse().unwrap())
    //     .build();
    //
    // println!("{:#?}", message);
}
