use libjose::error::Result;
use libjose::jwe::Decoder;
use libjose::jwe::Encoder;
use libjose::jwe::JweAlgorithm;
use libjose::jwe::JweEncryption;
use libjose::jwe::JweFormat;
use libjose::jwe::JweHeader;
use libjose::jwe::Token;
use libjose::jwk::Jwk;

static CLAIMS: &[u8] = b"libjose";

fn roundtrip(algorithm: JweAlgorithm, encryption: JweEncryption) -> Result<()> {
  let header: JweHeader = JweHeader::new(algorithm, encryption);

  let secret: Jwk = Jwk::random((algorithm, encryption))?;
  let public: Jwk = secret.to_public();

  let secret2: Jwk = Jwk::random((algorithm, encryption))?;
  let public2: Jwk = secret2.to_public();

  let mut encoder: Encoder = Encoder::new().protected(&header).secret(&secret2).recipient(&public);
  let mut decoder: Decoder = Decoder::new(&secret).public(&public2);

  let encoded: String = encoder.encode(CLAIMS)?;
  let decoded: Token = decoder.decode(encoded.as_bytes())?;

  assert_eq!(decoded.0.alg(), header.alg());
  assert_eq!(decoded.0.enc(), header.enc());
  assert_eq!(decoded.1, CLAIMS);

  encoder = encoder.format(JweFormat::General);
  decoder = decoder.format(JweFormat::General);

  let encoded: String = encoder.encode(CLAIMS)?;
  let decoded: Token = decoder.decode(encoded.as_bytes())?;

  assert_eq!(decoded.0.alg(), header.alg());
  assert_eq!(decoded.0.enc(), header.enc());
  assert_eq!(decoded.1, CLAIMS);

  encoder = encoder.format(JweFormat::Flatten);
  decoder = decoder.format(JweFormat::Flatten);

  let encoded: String = encoder.encode(CLAIMS)?;
  let decoded: Token = decoder.decode(encoded.as_bytes())?;

  assert_eq!(decoded.0.alg(), header.alg());
  assert_eq!(decoded.0.enc(), header.enc());
  assert_eq!(decoded.1, CLAIMS);

  Ok(())
}

#[test]
fn test_roundtrip() {
  for alg in JweAlgorithm::ALL {
    for enc in JweEncryption::ALL {
      roundtrip(*alg, *enc).unwrap();
    }
  }
}
