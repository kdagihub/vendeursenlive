use async_trait::async_trait;
use redis::AsyncCommands;
use uuid::Uuid;

use crate::application::{
    errors::ApplicationError,
    ports::auth::{PhoneOtpChallenge, PhoneOtpChallengeStore},
};

#[derive(Debug, Clone)]
pub struct RedisPhoneOtpChallengeStore {
    client: redis::Client,
}

impl RedisPhoneOtpChallengeStore {
    pub fn new(client: redis::Client) -> Self {
        Self { client }
    }

    fn challenge_key(id: Uuid) -> String {
        format!("auth:phone-otp:challenge:{id}")
    }

    fn attempts_key(id: Uuid) -> String {
        format!("auth:phone-otp:attempts:{id}")
    }

    fn cooldown_key(phone_number: &str) -> String {
        format!("auth:phone-otp:cooldown:{phone_number}")
    }

    fn active_key(phone_number: &str) -> String {
        format!("auth:phone-otp:active:{phone_number}")
    }

    async fn connection(&self) -> Result<redis::aio::MultiplexedConnection, ApplicationError> {
        self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(redis_error)
    }
}

#[async_trait]
impl PhoneOtpChallengeStore for RedisPhoneOtpChallengeStore {
    async fn reserve_send(
        &self,
        phone_number: &str,
        ttl_seconds: i64,
    ) -> Result<bool, ApplicationError> {
        let mut connection = self.connection().await?;
        let result: Option<String> = redis::cmd("SET")
            .arg(Self::cooldown_key(phone_number))
            .arg("1")
            .arg("NX")
            .arg("EX")
            .arg(ttl_seconds)
            .query_async(&mut connection)
            .await
            .map_err(redis_error)?;
        Ok(result.is_some())
    }

    async fn release_send(&self, phone_number: &str) -> Result<(), ApplicationError> {
        let mut connection = self.connection().await?;
        connection
            .del::<_, ()>(Self::cooldown_key(phone_number))
            .await
            .map_err(redis_error)
    }

    async fn save(
        &self,
        challenge: &PhoneOtpChallenge,
        ttl_seconds: i64,
    ) -> Result<(), ApplicationError> {
        let payload = serde_json::to_string(challenge)
            .map_err(|error| ApplicationError::Infrastructure(error.to_string()))?;
        let mut connection = self.connection().await?;
        let active_key = Self::active_key(&challenge.phone_number);
        let previous_challenge_id: Option<String> =
            connection.get(&active_key).await.map_err(redis_error)?;
        if let Some(previous_challenge_id) =
            previous_challenge_id.and_then(|value| Uuid::parse_str(&value).ok())
        {
            let previous_keys = [
                Self::challenge_key(previous_challenge_id),
                Self::attempts_key(previous_challenge_id),
            ];
            connection
                .del::<_, ()>(&previous_keys)
                .await
                .map_err(redis_error)?;
        }
        connection
            .set_ex::<_, _, ()>(
                Self::challenge_key(challenge.id),
                payload,
                ttl_seconds as u64,
            )
            .await
            .map_err(redis_error)?;
        connection
            .set_ex::<_, _, ()>(active_key, challenge.id.to_string(), ttl_seconds as u64)
            .await
            .map_err(redis_error)
    }

    async fn find(
        &self,
        challenge_id: Uuid,
    ) -> Result<Option<PhoneOtpChallenge>, ApplicationError> {
        let mut connection = self.connection().await?;
        let payload: Option<String> = connection
            .get(Self::challenge_key(challenge_id))
            .await
            .map_err(redis_error)?;
        payload
            .map(|payload| serde_json::from_str(&payload))
            .transpose()
            .map_err(|error| ApplicationError::Infrastructure(error.to_string()))
    }

    async fn record_failed_attempt(
        &self,
        challenge_id: Uuid,
        ttl_seconds: i64,
    ) -> Result<u32, ApplicationError> {
        let mut connection = self.connection().await?;
        let key = Self::attempts_key(challenge_id);
        let attempts: u32 = connection.incr(&key, 1_u32).await.map_err(redis_error)?;
        connection
            .expire::<_, ()>(&key, ttl_seconds)
            .await
            .map_err(redis_error)?;
        Ok(attempts)
    }

    async fn delete(&self, challenge_id: Uuid) -> Result<(), ApplicationError> {
        let mut connection = self.connection().await?;
        let keys = [
            Self::challenge_key(challenge_id),
            Self::attempts_key(challenge_id),
        ];
        connection.del::<_, ()>(&keys).await.map_err(redis_error)
    }
}

fn redis_error(error: redis::RedisError) -> ApplicationError {
    ApplicationError::Infrastructure(format!("OTP challenge storage failed: {error}"))
}
