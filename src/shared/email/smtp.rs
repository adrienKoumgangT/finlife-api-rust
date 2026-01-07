use lettre::{
    message::{Mailbox, Message}, 
    transport::smtp::authentication::Credentials, 
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor
};

#[derive(Clone)]
pub struct SmtpMailer {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
    from_name: String,
}

impl SmtpMailer {
    pub fn from_env() -> anyhow::Result<Self> {
        let host = std::env::var("SMTP_HOST")?;
        let username = std::env::var("SMTP_USERNAME")?;
        let password = std::env::var("SMTP_PASSWORD")?;
        let from_email = std::env::var("SMTP_FROM_EMAIL")?;
        let from_name = std::env::var("SMTP_FROM_NAME").unwrap_or_else(|_| "MyFinance".into());

        let creds = Credentials::new(username, password);
        let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&host)?
            .credentials(creds)
            .build();

        Ok(Self { transport, from_email, from_name })
    }

    pub async fn send(&self, to: &str, subject: &str, body_text: Option<&str>, body_html: Option<&str>) -> anyhow::Result<()> {
        // Simple version: send as plain text if exists, else html as text fallback
        let content = body_text
            .or(body_html)
            .unwrap_or("");

        let email = Message::builder()
            .from(Mailbox::new(Some(self.from_name.clone()), self.from_email.parse()?))
            .to(to.parse()?)
            .subject(subject)
            .body(content.to_string())?;

        self.transport.send(email).await?;
        Ok(())
    }
}
