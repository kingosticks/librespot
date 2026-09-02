#![warn(missing_docs)]
#![doc=include_str!("../README.md")]

#[macro_use]
extern crate log;

use librespot_core as core;
use librespot_protocol as protocol;

mod context_resolver;
mod model;
mod player;
mod shuffle_vec;
mod spirc;
mod state;

mod playback {
    pub use librespot_playback::mixer;

    pub mod player {
        pub use librespot_playback::player::{PlayerEvent, PlayerEventChannel};

        pub type Player = dyn crate::PlayerController;
    }
}

pub use model::*;
pub use player::*;
pub use spirc::*;
pub use state::*;
