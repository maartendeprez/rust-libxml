fn main() {
  // declare availability of config variable (without setting it)
  println!("cargo::rustc-check-cfg=cfg(libxml_older_than_2_12)");

  let mut version_iter = str::from_utf8(libxml_sys::bindings::LIBXML_DOTTED_VERSION)
    .unwrap()
    .split('.');
  let major: u8 = version_iter.next().unwrap().parse().unwrap();
  let minor: u8 = version_iter.next().unwrap().parse().unwrap();

  if (major, minor) < (2, 12) {
    println!("cargo::rustc-cfg=libxml_older_than_2_12");
  }
}
