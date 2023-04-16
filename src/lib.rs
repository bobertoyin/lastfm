//! `lastfm` is an async Rust client to fetch your <Last.fm> listening history or the track you are currently playing
//!
//! ## Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! lastfm = "0.1.0"
//! ```
//!
//! Alternatively you can run:
//!
//! ```bash
//! cargo add lastfm
//! ````
//!
//! ## Usage
//!
//! To use this library you will need a Last.fm account and an API key.
//! You can get one by [registering an application](https://www.last.fm/api/account/create).
//! If you have already registered an application, you can find your API key in the [API settings](https://www.last.fm/api/accounts).
//!
//! ### Create a new client
//!
//! ```rust
//! # use lastfm::Client;
//! #
//! let client = Client::new("YOUR_API_KEY", "YOUR_USERNAME");
//! ```
//!
//! If you have your API key exposed through the `LASTFM_API_KEY` environment variable, you can use the `from_env` method:
//!
//! ```rust
//! # use lastfm::Client;
//! #
//! let client = Client::from_env("YOUR_USERNAME");
//! ```
//!
//! Note: this method will panic if `LASTFM_API_KEY` is not set.
//!
//! ### Fetch the track you are currently playing
//!
//! ```rust,no_run
//! # use lastfm::Client;
//! #
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   let client = Client::new("YOUR_API_KEY", "YOUR_USERNAME");
//!   let now_playing = client.now_playing().await?;
//!   if let Some(track) = now_playing {
//!     println!("Now playing: {} - {}", track.artist.name, track.name);
//!   }
//!
//!   Ok(())
//! }
//! ```
//!
//! ### Fetch your listening history
//!
//! **Note**: You will need the `futures-util` crate to use the `Stream` returned by `all_tracks`.
//!
//!
//! ```rust,no_run
//! use futures_util::pin_mut;
//! # use lastfm::Client;
//! #
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   let client = Client::new("YOUR_API_KEY", "YOUR_USERNAME");
//!   let tracks = client.all_tracks().await?;
//!   println!("Total tracks: {}", tracks.total_tracks);
//!
//!    let recent_tracks = tracks.into_stream();
//!    pin_mut!(recent_tracks); // needed for iteration
//!    while let Some(track) = recent_tracks.next().await {
//!        match track {
//!            Ok(track) => {
//!                println!(
//!                    "{}: {} - {}",
//!                    track.date.to_rfc2822(),
//!                    track.artist.name,
//!                    track.name
//!                );
//!            }
//!            Err(e) => {
//!                println!("Error fetching data: {:?}", e);
//!            }
//!        }
//!    }
//!    Ok(())
//! }
//! ```

#[macro_use]
extern crate lazy_static;

pub mod artist;
pub mod client;
pub mod error_response;
pub mod errors;
pub mod imageset;
pub mod lfm_date;
pub mod recent_tracks_page;
pub mod retry_delay;
pub mod track;

pub use client::Client;