use async_trait::async_trait;
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use tracing::warn;

use crate::{
    application::{errors::ApplicationError, ports::auth::AuthEmailSender},
    infrastructure::config::EmailConfig,
};

#[derive(Debug, Clone)]
pub struct SmtpAuthEmailSender {
    config: EmailConfig,
}

impl SmtpAuthEmailSender {
    pub fn new(config: EmailConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl AuthEmailSender for SmtpAuthEmailSender {
    async fn send_password_reset(
        &self,
        to_email: &str,
        reset_url: &str,
    ) -> Result<(), ApplicationError> {
        if !self.config.delivery_enabled {
            warn!("email delivery is disabled; password reset email was not sent");
            return Ok(());
        }

        let message = Message::builder()
            .from(self.sender_mailbox()?)
            .to(parse_target(to_email)?)
            .subject("Réinitialisation de ton mot de passe VendeursEnLive")
            .header(ContentType::TEXT_HTML)
            .body(password_reset_body(reset_url))
            .map_err(|error| {
                ApplicationError::Infrastructure(format!("email build failed: {error}"))
            })?;

        self.send(message).await
    }

    async fn send_email_verification(
        &self,
        to_email: &str,
        verification_url: &str,
    ) -> Result<(), ApplicationError> {
        if !self.config.delivery_enabled {
            warn!("email delivery is disabled; verification email was not sent");
            return Ok(());
        }

        let message = Message::builder()
            .from(self.sender_mailbox()?)
            .to(parse_target(to_email)?)
            .subject("Vérifie ton adresse email VendeursEnLive")
            .header(ContentType::TEXT_HTML)
            .body(email_verification_body(verification_url))
            .map_err(|error| {
                ApplicationError::Infrastructure(format!("email build failed: {error}"))
            })?;

        self.send(message).await
    }
}

impl SmtpAuthEmailSender {
    fn sender_mailbox(&self) -> Result<Mailbox, ApplicationError> {
        format!("{} <{}>", self.config.from_name, self.config.from_email)
            .parse()
            .map_err(|error| {
                ApplicationError::Infrastructure(format!("invalid from email: {error}"))
            })
    }

    async fn send(&self, message: Message) -> Result<(), ApplicationError> {
        let smtp_host = required(self.config.smtp_host.as_deref(), "SMTP host")?;
        let smtp_username = required(self.config.smtp_username.as_deref(), "SMTP username")?;
        let smtp_password = required(self.config.smtp_password.as_deref(), "SMTP password")?;
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(smtp_host)
            .map_err(|error| {
                ApplicationError::Infrastructure(format!("SMTP setup failed: {error}"))
            })?
            .credentials(Credentials::new(
                smtp_username.to_owned(),
                smtp_password.to_owned(),
            ))
            .port(self.config.smtp_port)
            .build();

        mailer.send(message).await.map_err(|error| {
            ApplicationError::Infrastructure(format!("email send failed: {error}"))
        })?;
        Ok(())
    }
}

fn required<'a>(value: Option<&'a str>, name: &str) -> Result<&'a str, ApplicationError> {
    value.ok_or_else(|| ApplicationError::Infrastructure(format!("{name} is not configured")))
}

fn parse_target(to_email: &str) -> Result<Mailbox, ApplicationError> {
    to_email
        .parse()
        .map_err(|error| ApplicationError::Infrastructure(format!("invalid target email: {error}")))
}

fn password_reset_body(reset_url: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="fr">
  <body style="font-family:Arial,sans-serif;color:#17121f;line-height:1.5">
    <h1 style="font-size:22px">Réinitialisation du mot de passe</h1>
    <p>Tu as demandé à réinitialiser ton mot de passe VendeursEnLive.</p>
    <p>
      <a href="{reset_url}" style="display:inline-block;background:#ff3650;color:#fff;text-decoration:none;padding:12px 18px;border-radius:6px;font-weight:700">
        Choisir un nouveau mot de passe
      </a>
    </p>
    <p>Ce lien expire bientôt. Si tu n'es pas à l'origine de cette demande, ignore cet email.</p>
  </body>
</html>"#
    )
}

fn email_verification_body(verification_url: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="fr">
  <body style="font-family:Arial,sans-serif;color:#17121f;line-height:1.5">
    <h1 style="font-size:22px">Vérifie ton adresse email</h1>
    <p>Confirme ton adresse pour commander pendant les lives ou utiliser les outils vendeur.</p>
    <p>
      <a href="{verification_url}" style="display:inline-block;background:#ff3650;color:#fff;text-decoration:none;padding:12px 18px;border-radius:6px;font-weight:700">
        Vérifier mon email
      </a>
    </p>
    <p>Ce lien expire bientôt. Si tu n'as pas créé ce compte, ignore cet email.</p>
  </body>
</html>"#
    )
}
