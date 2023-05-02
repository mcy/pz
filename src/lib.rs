pub mod ir;
pub mod plugin;
pub mod report;
pub mod rt;
pub mod syn;

pub use rt::ptr::Mut;
pub use rt::ptr::View;
pub use rt::str::Str;
pub use rt::str::StrBuf;

pub mod proto {
  include!(concat!(env!("OUT_DIR"), "/pz.rs"));
  pub mod plugin {
    include!(concat!(env!("OUT_DIR"), "/pz.plugin.rs"));
  }
}

#[path = "lib.pz.rs"]
pub mod protoz;
