use std::marker::PhantomData;

use dotenvy::dotenv;

/// The uninitialized state of the environment.
pub struct EnvironmentUninitialized;
/// The initialized state of the environment.
pub struct EnvironmentInitialized;

/// Represents the environment configuration for the application.
pub struct Environment<Status = EnvironmentUninitialized> {
    state: PhantomData<Status>,
}

impl Environment<EnvironmentUninitialized> {
    /// Creates a new instance of the Environment struct.
    pub fn new() -> Environment<EnvironmentInitialized> {
        dotenv().ok();
        Environment { state: PhantomData }
    }
}

impl Environment<EnvironmentInitialized> {
    const APP_VAR_PREFIX: &'static str = "BLOCKLOCAL";
    const DATABASE_VAR_PREFIX: &'static str = "DATABASE";
    const GITHUB_VAR_PREFIX: &'static str = "GITHUB";

    const DEFAULT_TMP_DIR: &'static str = "/tmp/blocklocal";

    fn get_var(&self, key: &str) -> Option<String> {
        std::env::var(key).ok()
    }

    fn get_prefixed_var(&self, prefix: &str, key: &str) -> Option<String> {
        let full_key = format!("{}_{}", prefix, key);
        self.get_var(&full_key)
    }

    fn get_app_var(&self, key: &str) -> Option<String> {
        self.get_prefixed_var(Self::APP_VAR_PREFIX, key)
    }

    fn get_database_var(&self, key: &str) -> Option<String> {
        self.get_prefixed_var(Self::DATABASE_VAR_PREFIX, key)
    }

    fn get_github_var(&self, key: &str) -> Option<String> {
        self.get_prefixed_var(Self::GITHUB_VAR_PREFIX, key)
    }

    /// Returns the temporary directory path.
    pub fn get_app_tmp_dir(&self) -> String {
        self.get_app_var("TMP_DIR")
            .unwrap_or_else(|| Environment::<EnvironmentInitialized>::DEFAULT_TMP_DIR.to_string())
    }

    /// Returns the database URL.
    pub fn get_database_url(&self) -> String {
        self.get_database_var("URL")
            .expect("DATABASE_URL must be set")
    }

    /// Returns the GitHub API bearer token.
    pub fn get_github_bearer(&self) -> Option<String> {
        self.get_github_var("BEARER")
    }
}
