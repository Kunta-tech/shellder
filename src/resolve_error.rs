// Copyright (c) 2025 Saugata Kundu
// Licensed under the Apache-2.0 License

#[derive(Debug, thiserror::Error)]
pub enum ResolveError {
    #[error("No instance registered for the requested type")]
    NotFound,
    #[error("Failed to downcast the stored instance")]
    DowncastFailed,
}
