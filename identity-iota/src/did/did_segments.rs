use crate::did::IotaDID;

macro_rules! get {
  (@network $this:expr) => {
    &$this.0[..get!(@head $this)]
  };
  (@shard $this:expr) => {
    &$this.0[&get!(@head $this) + 1..get!(@tail $this)]
  };
  (@tag $this:expr) => {
    &$this.0[get!(@tail $this) + 1..]
  };
  (@head $this:expr) => {
    $this.0.find(':').unwrap()
  };
  (@tail $this:expr) => {
    $this.0.rfind(':').unwrap()
  };
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segments<'a>(pub(crate) &'a str);

impl<'a> Segments<'a> {
    pub fn is_default_network(&self) -> bool {
        match self.count() {
            1 => true,
            2 | 3 => get!(@network self) == IotaDID::DEFAULT_NETWORK,
            _ => unreachable!("Segments::is_default_network called for invalid IOTA DID"),
        }
    }

    pub fn network(&self) -> &'a str {
        match self.count() {
            1 => IotaDID::DEFAULT_NETWORK,
            2 | 3 => get!(@network self),
            _ => unreachable!("Segments::network called for invalid IOTA DID"),
        }
    }

    pub fn shard(&self) -> Option<&'a str> {
        match self.count() {
            1 | 2 => None,
            3 => Some(get!(@shard self)),
            _ => unreachable!("Segments::shard called for invalid IOTA DID"),
        }
    }

    pub fn tag(&self) -> &'a str {
        match self.count() {
            1 => self.0,
            2 | 3 => get!(@tag self),
            _ => unreachable!("Segments::tag called for invalid IOTA DID"),
        }
    }

    pub fn count(&self) -> usize {
        self.0.split(':').count()
    }
}
