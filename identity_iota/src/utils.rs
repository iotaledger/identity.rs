use core::{cmp::Ordering, iter::once};
use iota::{
    crypto::ternary::{
        sponge::{CurlP81, Sponge as _},
        Hash,
    },
    ternary::{raw::RawEncoding, Btrit, T1B1Buf, TritBuf, Trits, TryteBuf, T1B1},
    transaction::bundled::{Address, BundledTransaction, BundledTransactionField as _},
};
use iota_conversion::trytes_converter;

use crate::error::{Error, Result};

pub fn txn_hash(txn: &BundledTransaction) -> Hash {
    let mut curl: CurlP81 = CurlP81::new();
    let mut tbuf: TritBuf<T1B1Buf> = TritBuf::zeros(BundledTransaction::trit_len());

    txn.into_trits_allocated(&mut tbuf);

    Hash::from_inner_unchecked(curl.digest(&tbuf).expect("infallible"))
}

pub fn txn_hash_trytes(txn: &BundledTransaction) -> String {
    encode_trits(txn_hash(txn).as_trits())
}

pub fn encode_trits<T: ?Sized>(trits: &Trits<T>) -> String
where
    T: RawEncoding<Trit = Btrit>,
{
    trits.iter_trytes().map(char::from).collect()
}

pub fn create_address_from_trits(trits: impl AsRef<str>) -> Result<Address> {
    TryteBuf::try_from_str(trits.as_ref())
        .map_err(Into::into)
        .map(|trytes| trytes.as_trits().encode::<T1B1Buf>())
        .map(Address::from_inner_unchecked)
}

pub fn to_tryte(byte: u8) -> impl IntoIterator<Item = char> {
    once(iota_constants::TRYTE_ALPHABET[(byte % 27) as usize])
        .chain(once(iota_constants::TRYTE_ALPHABET[(byte / 27) as usize]))
}

pub fn utf8_to_trytes(input: impl AsRef<[u8]>) -> String {
    input.as_ref().iter().copied().flat_map(to_tryte).collect()
}

pub fn trytes_to_utf8(string: impl AsRef<str>) -> Result<String> {
    trytes_converter::to_string(string.as_ref()).map_err(|_| Error::InvalidTryteConversion)
}

pub fn bundles_from_trytes(mut transactions: Vec<BundledTransaction>) -> Vec<Vec<BundledTransaction>> {
    transactions.sort_by(|a, b| {
        // TODO: impl Ord for Address, Tag, Hash
        cmp_trits(a.address().to_inner(), b.address().to_inner())
            .then(cmp_trits(a.tag().to_inner(), b.tag().to_inner()))
            // different messages may have the same bundle hash!
            .then(cmp_trits(a.bundle().to_inner(), b.bundle().to_inner()))
            // reverse order of transactions will be extracted from back with `pop`
            .then(a.index().to_inner().cmp(b.index().to_inner()).reverse())
    });

    let mut bundles: Vec<Vec<BundledTransaction>> = Vec::new();

    if let Some(root) = transactions.pop() {
        let mut bundle: Vec<BundledTransaction> = vec![root];

        loop {
            if let Some(transaction) = transactions.pop() {
                if cmp_transaction(&bundle[0], &transaction) {
                    bundle.push(transaction);
                } else {
                    bundles.push(bundle);
                    bundle = vec![transaction];
                }
            } else {
                bundles.push(bundle);
                break;
            }
        }
    }

    // TODO: Check the bundles
    bundles
}

fn cmp_trits(a: &Trits<T1B1>, b: &Trits<T1B1>) -> Ordering {
    a.iter().cmp(b.iter())
}

fn cmp_transaction(a: &BundledTransaction, b: &BundledTransaction) -> bool {
    a.address() == b.address() && a.tag() == b.tag() && a.bundle() == b.bundle()
}
