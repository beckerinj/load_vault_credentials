use anyhow::Context;
use serde::Deserialize;
use vaultrs::{
    auth::approle,
    client::{Client, VaultClient, VaultClientSettingsBuilder},
    kv2,
};

#[derive(Deserialize)]
struct Env {
    vault_role_id: String,
    vault_secret_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub async fn load_credentials(mount: &str, path: &str) -> anyhow::Result<Credentials> {
    let env: Env = envy::from_env().context("failed to load vault credentials from env. expected to find $VAULT_ROLE_ID and $VAULT_SECRET_ID")?;

    let mut vault = VaultClient::new(VaultClientSettingsBuilder::default().build()?)?;

    let auth_info = approle::login(&vault, "approle", &env.vault_role_id, &env.vault_secret_id)
        .await
        .context("failed to login via approle")?;

    vault.set_token(&auth_info.client_token);

    kv2::read(&vault, mount, path).await.context(format!(
        "failed to load credentials | mount: {mount} | path: {path}"
    ))
}
