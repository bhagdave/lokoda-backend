use lettre::{
    transport::smtp::{authentication::Credentials, client::{Tls, TlsParameters}, SmtpTransport},
    Message, Transport,
};

use crate::configuration::*;

pub fn send_email(to: &str, from: &str, subject: &str, body: &str) {
    let configuration = get_configuration().expect("Failed to read configuration.");
    println!("Smtp Host:{}", &configuration.email.host);
    
    // Create email message using the new Message type
    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(String::from(body))
        .unwrap();

    let creds = Credentials::new(
        configuration.email.username.to_string(),
        configuration.email.password.to_string(),
    );

    let tls_parameters = TlsParameters::new(configuration.email.host.clone()).unwrap();

    // Create SMTP transport with TLS
    let mailer = SmtpTransport::relay(&configuration.email.host)
        .unwrap()
        .port(configuration.email.port)
        .credentials(creds)
        .tls(Tls::Required(tls_parameters))
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}


