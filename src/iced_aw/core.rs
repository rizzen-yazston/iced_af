//! `iced_aw_core`.
//use cfg_if::cfg_if;

//pub mod overlay;
//pub mod renderer;
pub mod icons;

//cfg_if! {
//    if #[cfg(feature = "icons")] {
//        pub use icons::{BOOTSTRAP_FONT, BOOTSTRAP_FONT_BYTES, NERD_FONT, NERD_FONT_BYTES, Bootstrap, Nerd, bootstrap, nerd};
//    } else {
        pub use icons::{BOOTSTRAP_FONT, BOOTSTRAP_FONT_BYTES, Bootstrap, bootstrap};
//    }
//}
