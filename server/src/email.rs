use lettre::{
    Address, AsyncFileTransport, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    address::Envelope,
    message::{Mailbox, header::ContentType},
    transport::{
        smtp::authentication::{Credentials, Mechanism},
        stub::AsyncStubTransport,
    },
};

use crate::config;

pub trait Email {
    fn subject(&self) -> String;
    fn body(&self) -> String;
}

pub struct Emails {
    backend: EmailBackend,
    pub domain: String,
    from: Address,
}

const DEFAULT_FROM: &str = "noreply@myjournly.com";

impl Emails {
    pub fn from_config(config: &config::Server) -> Self {
        let login = config.mailgun_smtp.smtp_login.clone();
        let password = config.mailgun_smtp.smtp_password.clone();
        let server = config.mailgun_smtp.smtp_server.clone();

        let from = login.as_deref().unwrap_or(DEFAULT_FROM).parse().unwrap();

        let backend = match (login, password, server) {
            (Some(login), Some(password), Some(server)) => {
                let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&server)
                    .unwrap()
                    .credentials(Credentials::new(login, password))
                    .authentication([Mechanism::Plain].to_vec())
                    .build();

                EmailBackend::Smtp(transport)
            }
            _ => {
                let transport = AsyncFileTransport::new("/tmp");
                EmailBackend::FileSystem(transport)
            }
        };

        if config.base.production && !matches!(backend, EmailBackend::Smtp { .. }) {
            panic!("Only Smtp backend is allowed in production.");
        }

        let domain = config.base.domain_name.clone();

        Self {
            backend,
            domain,
            from,
        }
    }

    fn build_message(
        &self,
        recipient: &str,
        subject: String,
        body: String,
    ) -> Result<Message, EmailError> {
        let from = Mailbox::new(Some(self.domain.clone()), self.from.clone());

        let message = Message::builder()
            .to(recipient.parse()?)
            .from(from)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;

        Ok(message)
    }

    pub async fn send<E: Email>(&self, recipient: &str, email: E) -> Result<(), EmailError> {
        let email = self.build_message(recipient, email.subject(), email.body())?;

        self.backend
            .send(email)
            .await
            .map_err(EmailError::TransportError)
    }

    /// Create a new test backend that stores all the outgoing emails in memory, allowing for tests
    /// to later assert the mails were sent.
    pub fn new_in_memory() -> Self {
        Self {
            backend: EmailBackend::Memory(AsyncStubTransport::new_ok()),
            domain: "myjournly.com".into(),
            from: DEFAULT_FROM.parse().unwrap(),
        }
    }

    /// This is supposed to be used only during tests, to retrieve the messages stored in the
    /// "memory" backend. It's not cfg'd away because our integration tests need to access this.
    pub async fn mails_in_memory(&self) -> Option<Vec<(Envelope, String)>> {
        if let EmailBackend::Memory(transport) = &self.backend {
            Some(transport.messages().await)
        } else {
            None
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error(transparent)]
    AddressError(#[from] lettre::address::AddressError),
    #[error(transparent)]
    MessageBuilderError(#[from] lettre::error::Error),
    #[error(transparent)]
    TransportError(anyhow::Error),
}

pub enum EmailBackend {
    // for production, will send emails using SMTP
    Smtp(AsyncSmtpTransport<Tokio1Executor>),
    // for local development, will store emails in /tmp directory
    FileSystem(AsyncFileTransport<Tokio1Executor>),
    // for tests, will keep emails in memory
    Memory(AsyncStubTransport),
}

impl EmailBackend {
    async fn send(&self, message: Message) -> anyhow::Result<()> {
        match self {
            EmailBackend::Smtp(transport) => transport.send(message).await.map(|_| ())?,
            EmailBackend::FileSystem(transport) => transport.send(message).await.map(|_| ())?,
            EmailBackend::Memory(transport) => transport.send(message).await.map(|_| ())?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tokio_test::{assert_err, assert_ok};

    use super::*;

    struct TestEmail;

    impl Email for TestEmail {
        fn subject(&self) -> String {
            "test".into()
        }

        fn body(&self) -> String {
            "test".into()
        }
    }

    #[actix_rt::test]
    async fn sending_to_invalid_email_fails() {
        let emails = Emails::new_in_memory();

        let address = "String.Format(\"{0}.{1}@live.com\", FirstName, LastName)";
        assert_err!(emails.send(address, TestEmail).await);
    }

    #[actix_rt::test]
    async fn sending_to_valid_email_succeeds() {
        let emails = Emails::new_in_memory();

        let address = "someone@example.com";
        assert_ok!(emails.send(address, TestEmail).await);
    }
}
