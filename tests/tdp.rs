#[path = "proto/lib.pz.rs"]
mod proto;

// Data in this file is generated using Protoscope.

#[test]
fn smoke() {
  let proto = proto::TestAll::parsed(&mut &[0x08, 0x01][..]).unwrap();
  dbg!(&proto);

  assert_eq!(proto.opt_i32(), 0);
}
