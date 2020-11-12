use anyhow::anyhow;
use core::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyRelation {
    VerificationMethod,
    Authentication,
    AssertionMethod,
    KeyAgreement,
    CapabilityInvocation,
    CapabilityDelegation,
}

impl Default for KeyRelation {
    fn default() -> Self {
        Self::VerificationMethod
    }
}

impl FromStr for KeyRelation {
    type Err = crate::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "VerificationMethod" => Ok(Self::VerificationMethod),
            "Authentication" => Ok(Self::Authentication),
            "AssertionMethod" => Ok(Self::AssertionMethod),
            "KeyAgreement" => Ok(Self::KeyAgreement),
            "CapabilityInvocation" => Ok(Self::CapabilityInvocation),
            "CapabilityDelegation" => Ok(Self::CapabilityDelegation),
            _ => Err(crate::Error::ParseError(anyhow!("Unknown KeyRelation"))),
        }
    }
}
