use pagino::Pagino;

#[test]
fn basic_test() -> () {
  let mut pagino = Pagino::new(true, true, true, true, 1, 10, 1, 1);

  let pages: Vec<i32> = pagino.get_pages();
  //[-1, -2, 1, 2, 3, 4, 5, -4, 10, -5, -6];
  assert_eq!(pages[0], -1);
  assert_eq!(pages[1], -2);
  assert_eq!(pages[2], 1);
  assert_eq!(pages[3], 2);
  assert_eq!(pages[4], 3);
  assert_eq!(pages[5], 4);
  assert_eq!(pages[6], 5);
  assert_eq!(pages[7], -4);
  assert_eq!(pages[8], 10);
  assert_eq!(pages[9], -5);
  assert_eq!(pages[10], -6);

  pagino.set_count(5);
  let pages: Vec<i32> = pagino.get_pages();
  //[-1, -2, 1, 2, 3, 4, 5, -5, -6];
  assert_eq!(pages[0], -1);
  assert_eq!(pages[1], -2);
  assert_eq!(pages[2], 1);
  assert_eq!(pages[3], 2);
  assert_eq!(pages[4], 3);
  assert_eq!(pages[5], 4);
  assert_eq!(pages[6], 5);
  assert_eq!(pages[7], -5);
  assert_eq!(pages[8], -6);
}

