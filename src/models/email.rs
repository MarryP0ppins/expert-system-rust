use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

use crate::config::Config;
use entity::users::UserModel;

pub struct Email {
    user: UserModel,
    url: String,
    from: String,
    config: Config,
}

impl Email {
    pub fn new(user: UserModel, url: String, config: Config) -> Self {
        let from = format!("ExpertSsytem <{}>", config.smtp_from.to_owned());

        Email {
            user,
            url,
            from,
            config,
        }
    }

    fn new_transport(&self) -> Result<SmtpTransport, lettre::transport::smtp::Error> {
        let creds = Credentials::new(
            self.config.smtp_user.to_owned(),
            self.config.smtp_pass.to_owned(),
        );

        let transport = SmtpTransport::relay(&self.config.smtp_host.to_owned())?
            .port(self.config.smtp_port)
            .credentials(creds)
            .build();

        Ok(transport)
    }

    async fn send_email(
        &self,
        template_name: &str,
        subject: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let email = Message::builder()
            .to(format!(
                "{} <{}>",
                self.user.username.as_str(),
                self.user.email.as_str()
            )
            .parse()
            .unwrap())
            .reply_to(self.from.as_str().parse().unwrap())
            .from(self.from.as_str().parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(format!("{}: \n {}", template_name, self.url))?;

        let transport = self.new_transport()?;

        transport.send(&email)?;
        Ok(())
    }

    pub async fn send_verification_code(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.send_email("verification_code", "Your account verification code")
            .await
    }

    pub async fn send_password_reset_token(
        &self,
        password_reset_token_expires_in: i64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.send_email(
            "reset_password",
            format!(
                "Your password reset token (valid for only {} minutes)",
                password_reset_token_expires_in
            )
            .as_str(),
        )
        .await
    }
}
