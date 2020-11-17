macro_rules! impl_bytes {
    ($ident:ident) => {
        #[derive(Clone)]
        pub struct $ident(Vec<u8>);

        impl $ident {
            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn to_hex(&self) -> String {
                $crate::utils::encode_hex(self)
            }
        }

        impl From<Vec<u8>> for $ident {
            fn from(other: Vec<u8>) -> $ident {
                Self(other)
            }
        }

        impl AsRef<[u8]> for $ident {
            fn as_ref(&self) -> &[u8] {
                self.0.as_slice()
            }
        }

        impl Drop for $ident {
            fn drop(&mut self) {
                use ::zeroize::Zeroize;
                self.0.zeroize();
            }
        }

        impl ::zeroize::Zeroize for $ident {
            fn zeroize(&mut self) {
                self.0.zeroize();
            }
        }

        impl ::core::fmt::Debug for $ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                write!(f, "{:?}", self.to_hex())
            }
        }

        impl ::core::fmt::Display for $ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                write!(f, "{}", self.to_hex())
            }
        }
    };
}

impl_bytes!(PublicKey);
impl_bytes!(SecretKey);
