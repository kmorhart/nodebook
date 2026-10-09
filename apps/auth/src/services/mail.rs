use lettre::message::{Mailbox, header::ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Message, SmtpTransport, Transport};

use crate::errors::AppError;

pub struct MailService;

impl MailService {
    pub fn send_verification(to: &str, token: &str) -> Result<(), AppError> {
        let smtp_host: String = std::env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let smtp_username: String = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
        let smtp_password: String = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

        let public_url: String = std::env::var("PUBLIC_URL").expect("PUBLIC_URL must be set");

        let from_mailbox = smtp_username.parse::<Mailbox>()
            .map_err(|e| AppError::Internal("Invalid sender email configuration", Some(e.to_string())))?;

        let to_mailbox = to.parse::<Mailbox>()
            .map_err(|e| AppError::Internal("Invalid recipient email format", Some(e.to_string())))?;


        let tls_parameters = TlsParameters::builder("mail.morhart.dev".to_string())
            .dangerous_accept_invalid_certs(true)
            .build()
            .unwrap();

        let credentials = Credentials::new(smtp_username.clone(), smtp_password.clone());

        let email = Message::builder()
            .from(from_mailbox)
            .to(to_mailbox)
            .subject("Please verify your email")
            .header(ContentType::TEXT_PLAIN)
            .body("Please verify your email by clicking the following link: ".to_string() + &public_url + "/verify/" + token)
            .map_err(|e| AppError::Internal("Failed to build email", Some(e.to_string())))?;

        let mailer = SmtpTransport::relay(&smtp_host)
            .unwrap()
            .credentials(credentials)
            .tls(Tls::Wrapper(tls_parameters))
            .build();

        mailer.send(&email)
            .map_err(|e| AppError::Internal("Failed to send email ", Some(e.to_string())))?;
        

        Ok(())
    }
}