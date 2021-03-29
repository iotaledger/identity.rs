pub trait EnvelopeExt {
  const FEXT: &'static str;
  const MIME: &'static str;

  fn as_bytes(&self) -> &[u8];
}
