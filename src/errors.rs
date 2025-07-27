// Copyright (c) 2025 Saugata Kundu - kundusaugata576@gmail.com
// Licensed under the Apache-2.0 License

#[derive(Debug, thiserror::Error)]
pub enum ResolveError {
    #[error("No instance registered for the requested type: ({0})")]
    NotFound(&'static str),
    #[error("Failed to downcast the stored instance: ({0})")]
    DowncastFailed(&'static str),
}

#[derive(Debug, thiserror::Error)]
pub enum RegistrationError {
    #[error("Type already registered: {0}")]
    AlreadyRegistered(&'static str),
}
