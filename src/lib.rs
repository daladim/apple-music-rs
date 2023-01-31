//! This crate makes it possible to control the local instance of Apple Music or iTunes.
//!
//! It contains both low-level, safe bindings over various APIs (see below), and higher-level helper functions.
//!
//! ## Abilities
//!
//! This crate is able to **read** info (about playlists, songs, etc.) from the local Apple Music instance.<br/>
//! It is also able to **edit** data (add songs to playlists, change track ratings, etc.) on the local Apple Music instance.<br/>
//! It is **not** meant to read or edit "cloud" playlists, or to do anything network-related.
//!
//! ## Underlying APIs used
//!
//! * On macOS, it uses AppleScript to control Apple Music
//! * On Windows, it uses COM API to control iTunes
//! * On Windows, there is no COM API (yet?) to control Apple Music
//!
//! ## Acknowledgements
//!
//! I am grateful to:
//! * [joshkunz](https://github.com/joshkunz) for providing [the first doc about iTunes COM API I discovered](https://www.joshkunz.com/iTunesControl/interfaceIiTunes.html)
//! * [DDRBoxman](https://github.com/DDRBoxman) for [wrappers around this COM API](https://github.com/DDRBoxman/itunes-rs) that I re-used to start building this crate.
