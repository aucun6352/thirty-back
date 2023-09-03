
use securestore::{KeySource, SecretsManager};

pub fn secrets_manager() -> SecretsManager {
  SecretsManager::load("secrets.json", KeySource::File("secrets.key")).unwrap()
}
