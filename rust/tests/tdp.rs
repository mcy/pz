#[path = "proto/lib.pz.rs"]
mod proto;

// Data in this file is generated using Protoscope.
//
// Eventually we'll use a Rust implementation of Protoscope to generate test
// data on-the-fly.
//
// For now, inputs need to be pasted into `protoscope -s | xxd -i`.

#[test]
fn smoke() {
  let _protoscope = r#"
    1: 42
    2: 42i32
    4: 200
    3: 600i64
    5: 777.77i32
    6: -inf64
    7: {"a normal-looking string"}
    8: false

    10: !{
      7: {"a nasty str\xffing"}
      10: {}
    }
    11: { 1: -1 }

    28: false
    
    21: 1
    21: 2
    22: -1i64
    21: 3
    22: -2
    23: 0i32
    24: 1000000
    24: 1000000
    24: 1000000
    23: 1000000
    24: 1000000
    25: inf32
    26: inf64
    25: 42.0i32
    25: -0.0i32

    27: {"more"}
    27: {"nor\x00mal"}
    27: {"strings?"}

    28: 2
    28: 1

    30: { 1: 1 }
    30: !{ 1: 6 }
    30: { 2: 9 }
    31: { 2: {"yet"} 2: {"more"} 2: {"hellropes"} }
  "#;

  let data = [
    0x08, 0x2a, 0x15, 0x2a, 0x00, 0x00, 0x00, 0x20, 0xc8, 0x01, 0x19, 0x58,
    0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2d, 0x48, 0x71, 0x42, 0x44,
    0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0xff, 0x3a, 0x17, 0x61,
    0x20, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x2d, 0x6c, 0x6f, 0x6f, 0x6b,
    0x69, 0x6e, 0x67, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x40, 0x00,
    0x53, 0x3a, 0x0f, 0x61, 0x20, 0x6e, 0x61, 0x73, 0x74, 0x79, 0x20, 0x73,
    0x74, 0x72, 0xff, 0x69, 0x6e, 0x67, 0x52, 0x00, 0x54, 0x5a, 0x0b, 0x08,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0xe0, 0x01,
    0x00, 0xa8, 0x01, 0x01, 0xa8, 0x01, 0x02, 0xb1, 0x01, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xa8, 0x01, 0x03, 0xb0, 0x01, 0xfe, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0xbd, 0x01, 0x00, 0x00,
    0x00, 0x00, 0xc0, 0x01, 0xc0, 0x84, 0x3d, 0xc0, 0x01, 0xc0, 0x84, 0x3d,
    0xc0, 0x01, 0xc0, 0x84, 0x3d, 0xb8, 0x01, 0xc0, 0x84, 0x3d, 0xc0, 0x01,
    0xc0, 0x84, 0x3d, 0xcd, 0x01, 0x00, 0x00, 0x80, 0x7f, 0xd1, 0x01, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x7f, 0xcd, 0x01, 0x00, 0x00, 0x28,
    0x42, 0xcd, 0x01, 0x00, 0x00, 0x00, 0x80, 0xda, 0x01, 0x04, 0x6d, 0x6f,
    0x72, 0x65, 0xda, 0x01, 0x07, 0x6e, 0x6f, 0x72, 0x00, 0x6d, 0x61, 0x6c,
    0xda, 0x01, 0x08, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x73, 0x3f, 0xe0,
    0x01, 0x02, 0xe0, 0x01, 0x01, 0xf2, 0x01, 0x02, 0x08, 0x01, 0xf3, 0x01,
    0x08, 0x06, 0xf4, 0x01, 0xf2, 0x01, 0x02, 0x10, 0x09, 0xfa, 0x01, 0x16,
    0x12, 0x03, 0x79, 0x65, 0x74, 0x12, 0x04, 0x6d, 0x6f, 0x72, 0x65, 0x12,
    0x09, 0x68, 0x65, 0x6c, 0x6c, 0x72, 0x6f, 0x70, 0x65, 0x73,
  ];

  let proto = dbg!(proto::TestAll::from_pb(&mut &data[..]).unwrap());

  assert_eq!(proto.opt_i32(), 42);
  assert_eq!(proto.opt_i64(), 42);
  assert_eq!(proto.opt_u32(), 600);
  assert_eq!(proto.opt_u64(), 200);
  assert_eq!(proto.opt_f32(), 777.77);
  assert_eq!(proto.opt_f64(), -f64::INFINITY);
  assert_eq!(proto.opt_str(), "a normal-looking string");
  assert_eq!(proto.opt_bool(), false);
  assert_eq!(proto.opt_recursive().opt_str(), b"a nasty str\xffing");
  assert!(proto.opt_recursive().opt_recursive_or().is_some());
  assert_eq!(proto.opt_nested().a(), -1);
  assert_eq!(proto.rep_i32(), [1, 2, 3]);
  assert_eq!(proto.rep_i64(), [-1, -2]);
  assert_eq!(proto.rep_u32(), [0, 1000000]);
  assert_eq!(proto.rep_u64(), [1000000; 4]);
  assert_eq!(proto.rep_f32(), [f32::INFINITY, 42.0, -0.0]);
  assert_eq!(proto.rep_f64(), [f64::INFINITY]);
  assert_eq!(proto.rep_bool(), [false, true, true]);
  assert_eq!(proto.rep_str(), ["more", "nor\0mal", "strings?"]);
  assert_eq!(proto.rep_recursive_at(0).opt_i32(), 1);
  assert_eq!(proto.rep_recursive_at(1).opt_i32(), 6);
  assert_eq!(proto.rep_recursive_at(2).opt_i64(), 9);
  assert_eq!(proto.rep_nested_at(0).b(), ["yet", "more", "hellropes"]);
}

#[test]
fn smoke_choice() {
  let _protoscope = r#"
    1: 42
  "#;

  let data = [0x08, 0x2a];

  let proto = dbg!(proto::TestAll2::from_pb(&mut &data[..]).unwrap());

  assert_eq!(proto.opt_i32_or(), Some(42));
  assert!(matches!(proto.cases(), proto::TestAll2Cases::OptI32(42)));

  let _protoscope = r#"
    7: {"a normal-looking string"}
    1: 42
    27: {"more"}
    27: {"nor\x00mal"}
    27: {"strings?"}
  "#;

  let data = [
    0x3a, 0x17, 0x61, 0x20, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x2d, 0x6c,
    0x6f, 0x6f, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e,
    0x67, 0x08, 0x2a, 0xda, 0x01, 0x04, 0x6d, 0x6f, 0x72, 0x65, 0xda, 0x01,
    0x07, 0x6e, 0x6f, 0x72, 0x00, 0x6d, 0x61, 0x6c, 0xda, 0x01, 0x08, 0x73,
    0x74, 0x72, 0x69, 0x6e, 0x67, 0x73, 0x3f,
  ];

  let proto = dbg!(proto::TestAll2::from_pb(&mut &data[..]).unwrap());

  assert!(proto.opt_str_or().is_none());
  assert!(proto.opt_i32_or().is_none());
  assert_eq!(proto.rep_str(), ["more", "nor\0mal", "strings?"]);
  match proto.cases() {
    proto::TestAll2Cases::RepStr(s) => {
      assert_eq!(s, ["more", "nor\0mal", "strings?"])
    }
    _ => panic!(),
  };
}
