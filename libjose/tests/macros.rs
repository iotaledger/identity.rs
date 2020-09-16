#[macro_export]
macro_rules! test_getset {
  ($ty:ty, $get:ident, $set:ident,Url = $value:expr) => {
    let mut header = <$ty>::new();
    assert_eq!(header.$get(), None);
    header.$set(::url::Url::parse($value).unwrap());
    assert_eq!(header.$get().unwrap().as_str(), $value);
  };
  ($ty:ty, $get:ident, $set:ident,Option = $value:expr) => {
    let mut header = <$ty>::new();
    assert_eq!(header.$get(), None);
    header.$set($value);
    assert_eq!(header.$get().unwrap(), $value);
  };
  ($ty:ty, $get:ident, $set:ident,OptionRef = $value:expr) => {
    let mut header = <$ty>::new();
    assert_eq!(header.$get(), None);
    header.$set($value);
    assert_eq!(header.$get().unwrap(), &$value);
  };
  ($ty:ty, $get:ident, $set:ident, $value:expr) => {
    assert!($value != Default::default());
    let mut header = <$ty>::new();
    assert_eq!(header.$get(), Default::default());
    header.$set($value);
    assert_eq!(header.$get(), $value);
  };
}
