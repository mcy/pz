pub mod ir;
pub mod plugin;
pub mod report;
pub mod syn;

pub mod proto {
  include!(concat!(env!("OUT_DIR"), "/pz.rs"));
  pub mod plugin {
    include!(concat!(env!("OUT_DIR"), "/pz.plugin.rs"));
  }
}

pub mod protoz {
  //include!("lib.pz.rs");
}
