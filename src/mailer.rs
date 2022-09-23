use lettre::address::AddressError as LettreAddressError;
use lettre::error::Error as LettreError;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::transport::smtp::Error as LettreSmtpError;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub enum MailerError {
    Email(LettreError),
    Address(LettreAddressError),
    SmtpTransport(LettreSmtpError),
}

impl std::error::Error for MailerError {}

impl fmt::Display for MailerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MailerError::Email(err) => write!(f, "{err}"),
            MailerError::Address(err) => write!(f, "{err}"),
            MailerError::SmtpTransport(err) => write!(f, "{err}"),
        }
    }
}

impl From<LettreError> for MailerError {
    fn from(error: LettreError) -> Self {
        MailerError::Email(error)
    }
}

impl From<LettreAddressError> for MailerError {
    fn from(error: LettreAddressError) -> Self {
        MailerError::Address(error)
    }
}

impl From<LettreSmtpError> for MailerError {
    fn from(error: LettreSmtpError) -> Self {
        MailerError::SmtpTransport(error)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MailOptions {
    pub from: String,
    pub reply_to: String,
    pub to: String,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SmtpCredentials {
    pub username: String,
    pub password: String,
}

impl From<SmtpCredentials> for Credentials {
    fn from(credentials: SmtpCredentials) -> Self {
        Credentials::new(credentials.username, credentials.password)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SmtpConnectionOptions {
    pub host: String,
    pub credentials: SmtpCredentials,
}

pub fn parse_mail_options(options: MailOptions) -> Result<Message, MailerError> {
    let message = Message::builder()
        .from(options.from.parse()?)
        .reply_to(options.reply_to.parse()?)
        .to(options.to.parse()?)
        .subject(options.subject)
        .body(options.body)?;

    Ok(message)
}

pub async fn send_mail_smtp(
    message: Message,
    connection_options: SmtpConnectionOptions,
) -> Result<Response, MailerError> {
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&connection_options.host)?
            .credentials(connection_options.credentials.into())
            .build();

    Ok(mailer.send(message).await?)
}
