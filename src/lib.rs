//! Korrektor library provides functionality to ease processing of Uzbek language.
//!
//! This library provides functionality formerly owned by <https://korrektor.uz> website.
//! Such functionality as transliteration and correction of Uzbek text will
//! be provided only as an API without sharing code openly later.
//!
//! The goal of the crate is to assist developers
//! with natural processing of Uzbek language which
//! can be complicated by the cyrillic and latin
//! letters use and lack of accessible materials
//! for studying the language.
//!
//! Rewritten from PHP, this Rust crate utilized
//! regex and pcre crates in order to perform natural
//! language processing.
//!
//! # Upcoming
//! Public API with transliteration and correction functionality.
pub mod uzbek;
pub mod utils;
