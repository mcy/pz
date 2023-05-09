// Generated code for bootstrapping.

include!(concat!(env!("OUT_DIR"), "/pz.rs"));

pub mod plugin {
  include!(concat!(env!("OUT_DIR"), "/pz.plugin.rs"));
}

#[path = "lib.pz.rs"]
pub mod z;
