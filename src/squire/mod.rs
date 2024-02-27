use fernet::Fernet;

/// Module for the web data configuration that holds the secrets required by the application.
pub mod settings;
/// Module that initializes the logger and loads the configuration into a dedicated Struct.
pub mod startup;
/// Module for the functions that handle encryption/encoding and decryption/decoding.
pub mod secure;
/// Module for the function that logs the incoming connection information.
pub mod logger;
/// Module for the functions that yield an ASCII art to print during startup.
pub mod ascii_art;
/// Module for the CORS middleware configuration.
pub mod middleware;
/// Module for the function that converts the subtitles from `srt` to `vtt` file format.
pub mod subtitles;
/// Module for the functions that scans the source and renders the filenames as a struct.
pub mod content;
/// Module that handles the authentication and
pub mod authenticator;
/// Module that handles parsing command line arguments.
pub mod parser;

/// Create a [Fernet](https://docs.rs/fernet/latest/fernet/) object to encrypt and decrypt session token.
///
/// Generates a random key, that can be safely passed to `Fernet::new()`
///
/// # Returns
///
/// Random key as a `String`
pub fn fernet_key() -> String { Fernet::generate_key() }
