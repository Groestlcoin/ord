use super::*;

#[derive(Debug, PartialEq)]
pub enum Object {
  Address(Address),
  Hash([u8; 32]),
  InscriptionId(InscriptionId),
  Integer(u128),
  OutPoint(OutPoint),
  Sat(Sat),
  SatPoint(SatPoint),
}

impl FromStr for Object {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    use Representation::*;

    match Representation::from_str(s)? {
      Address => Ok(Self::Address(s.parse()?)),
      Decimal | Degree | Percentile | Name => Ok(Self::Sat(s.parse()?)),
      Hash => Ok(Self::Hash(
        groestlcoin::hashes::sha256::Hash::from_str(s)?.into_inner(),
      )),
      InscriptionId => Ok(Self::InscriptionId(s.parse()?)),
      Integer => Ok(Self::Integer(s.parse()?)),
      OutPoint => Ok(Self::OutPoint(s.parse()?)),
      SatPoint => Ok(Self::SatPoint(s.parse()?)),
    }
  }
}

impl Display for Object {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Address(address) => write!(f, "{address}"),
      Self::Hash(hash) => {
        for byte in hash {
          write!(f, "{byte:02x}")?;
        }
        Ok(())
      }
      Self::InscriptionId(inscription_id) => write!(f, "{inscription_id}"),
      Self::Integer(integer) => write!(f, "{integer}"),
      Self::OutPoint(outpoint) => write!(f, "{outpoint}"),
      Self::Sat(sat) => write!(f, "{sat}"),
      Self::SatPoint(satpoint) => write!(f, "{satpoint}"),
    }
  }
}

impl Serialize for Object {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.collect_str(self)
  }
}

impl<'de> Deserialize<'de> for Object {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    Ok(DeserializeFromStr::deserialize(deserializer)?.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn from_str() {
    #[track_caller]
    fn case(s: &str, expected: Object) {
      let actual = s.parse::<Object>().unwrap();
      assert_eq!(actual, expected);
      let round_trip = actual.to_string().parse::<Object>().unwrap();
      assert_eq!(round_trip, expected);
    }

    assert_eq!(
      "nvtdijuwxlp".parse::<Object>().unwrap(),
      Object::Sat(Sat(8400000002310000))
    );
    assert_eq!("a".parse::<Object>().unwrap(), Object::Sat(Sat::LAST));
    assert_eq!(
      "1.1".parse::<Object>().unwrap(),
      Object::Sat(Sat(50 * COIN_VALUE + 1))
    );
    assert_eq!(
      "1°0′0″0‴".parse::<Object>().unwrap(),
      Object::Sat(Sat(10335937500000000))
    );
    assert_eq!("0%".parse::<Object>().unwrap(), Object::Sat(Sat(0)));

    case("0", Object::Integer(0));

    case(
      "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdefi1",
      Object::InscriptionId(
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdefi1"
          .parse()
          .unwrap(),
      ),
    );

    case(
      "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
      Object::Hash([
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd,
        0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
        0xcd, 0xef,
      ]),
    );
    case(
      "grs1qw508d6qejxtdg4y5r3zarvary0c5xw7k3k4sj5",
      Object::Address(
        "grs1qw508d6qejxtdg4y5r3zarvary0c5xw7k3k4sj5"
          .parse()
          .unwrap(),
      ),
    );
    case(
      "GRS1QW508D6QEJXTDG4Y5R3ZARVARY0C5XW7K3K4SJ5",
      Object::Address(
        "GRS1QW508D6QEJXTDG4Y5R3ZARVARY0C5XW7K3K4SJ5"
          .parse()
          .unwrap(),
      ),
    );
    case(
      "tgrs1qqqqqp399et2xygdj5xreqhjjvcmzhxw4aywxecjdzew6hylgvsess668a6",
      Object::Address(
        "tgrs1qqqqqp399et2xygdj5xreqhjjvcmzhxw4aywxecjdzew6hylgvsess668a6"
          .parse()
          .unwrap(),
      ),
    );
    case(
      "TGRS1QQQQQP399ET2XYGDJ5XREQHJJVCMZHXW4AYWXECJDZEW6HYLGVSESS668A6",
      Object::Address(
        "TGRS1QQQQQP399ET2XYGDJ5XREQHJJVCMZHXW4AYWXECJDZEW6HYLGVSESS668A6"
          .parse()
          .unwrap(),
      ),
    );
    case(
      "grsrt1qs758ursh4q9z627kt3pp5yysm78ddny6dv3fws",
      Object::Address(
        "grsrt1qs758ursh4q9z627kt3pp5yysm78ddny6dv3fws"
          .parse()
          .unwrap(),
      ),
    );
    case(
      "GRSRT1QS758URSH4Q9Z627KT3PP5YYSM78DDNY6DV3FWS",
      Object::Address(
        "GRSRT1QS758URSH4Q9Z627KT3PP5YYSM78DDNY6DV3FWS"
          .parse()
          .unwrap(),
      ),
    );
    case(
      "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef:123",
      Object::OutPoint(
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef:123"
          .parse()
          .unwrap(),
      ),
    );
    case(
      "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF:123",
      Object::OutPoint(
        "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF:123"
          .parse()
          .unwrap(),
      ),
    );
    case(
      "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef:123:456",
      Object::SatPoint(
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef:123:456"
          .parse()
          .unwrap(),
      ),
    );
    case(
      "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF:123:456",
      Object::SatPoint(
        "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF:123:456"
          .parse()
          .unwrap(),
      ),
    );
  }
}
