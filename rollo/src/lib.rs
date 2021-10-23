#![cfg_attr(docsrs, deny(rustdoc::broken_intra_doc_links))]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![doc = include_str!("../README.md")]

#[macro_use]
#[doc(hidden)]
pub(crate) mod macros;

cfg_server! {
    mod io;
}

#[doc(hidden)]
#[cfg(any(feature = "flatbuffers_helpers"))]
pub extern crate lazy_static;

pub mod error;

cfg_flatbuffers_helpers! {
    pub mod flatbuffers_helpers;
}

cfg_game! {
    pub mod game;
}

cfg_server! {
    pub mod packet;
    pub mod server;
    pub use async_trait::async_trait;
    pub use bytes;
    pub use tokio;
    pub use crossbeam::atomic::AtomicCell;

    cfg_macros! {
        /// ## Implement WorldTime
        /// ### Examples
        /// ```rust, no_run
        /// use std::sync::atomic::AtomicI64;
        ///
        /// let world = MyWorld {
        ///     time: AtomicI64::new(0)
        /// };
        ///
        /// let time = world.time();
        ///
        /// #[rollo::world_time]
        /// struct MyWorld {}
        /// ```
        pub use rollo_macros::world_time;
    }
}
