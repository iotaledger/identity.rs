use std::collections::{binary_heap::BinaryHeap, BTreeMap};

use serde::{Deserialize, Serialize};

use crate::storage::compress::tree::HTree;

mod tree;

#[derive(Serialize, Deserialize, Clone)]
pub struct HuffmanCodec {
    root: HTree,
}

impl HuffmanCodec {
    fn new(map: BTreeMap<char, u32>) -> HuffmanCodec {
        let mut heap = BinaryHeap::new();

        map.iter().for_each(|(c, v)| {
            let new = HTree::new(*c, *v);
            heap.push(new);
        });

        let mut left: HTree;
        let mut right: HTree;

        while heap.len() >= 2 {
            left = heap.pop().unwrap();
            right = heap.pop().unwrap();

            heap.push(HTree::merge(left, right));
        }

        HuffmanCodec {
            root: heap.pop().unwrap(),
        }
    }

    pub fn compress(data: String) -> crate::Result<Vec<u8>> {
        if data.len() == 1 {
            return Err(crate::Error::CompressionError(
                "Data must contain at least 2 characters to be compressed".into(),
            ));
        }
        let mut out: Vec<u8> = Vec::new();
        let frequency_map = frequency_map(&data);

        let tree = HuffmanCodec::new(frequency_map);

        let tree_bytes: Vec<u8> = bincode::serialize(&tree)?;
        let tree_size = tree_bytes.len().to_be_bytes();

        out.extend(&tree_size);
        out.extend(&tree_bytes);

        let mut map = BTreeMap::new();
        fill_code_map_outer(&mut map, &tree)?;

        let mut encoded: String = data.chars().fold(String::new(), |mut acc, c| {
            acc.push_str(map.get(&c).unwrap());
            acc
        });

        out.extend(&encoded.len().to_be_bytes());

        while encoded.len() % 8 != 0 {
            encoded.push_str("0");
        }

        let mut encoded_bytes: Vec<u8> = Vec::new();

        for _ in 0..encoded.len() / 8 {
            encoded_bytes.push(0u8);
        }

        encoded.char_indices().for_each(|(idx, ch)| {
            encoded_bytes[idx / 8] <<= 1;
            encoded_bytes[idx / 8] += (ch as u8) - b'0';
        });

        out.extend(encoded_bytes);

        Ok(out)
    }

    pub fn decompress(data: &[u8]) -> crate::Result<String> {
        let mut tree_size: [u8; 8] = [0; 8];
        tree_size[..8].clone_from_slice(&data[..8]);

        let tree_size_val = usize::from_be_bytes(tree_size);
        let encoded_tree = &data[8..(tree_size_val + 8)];
        let tree: HuffmanCodec = bincode::deserialize(encoded_tree)?;

        let mut byte_buf: [u8; 8] = [0; 8];
        for i in 0..8 {
            byte_buf[i] = data[i + (tree_size_val + 8)];
        }

        let data_size = usize::from_be_bytes(byte_buf);
        let encoded_data = &data[(tree_size_val + 16)..];

        let mut codec_clone = tree.clone();

        let mut output_string = String::from("");

        let mut bit_counter = 0;
        for byte in encoded_data {
            for i in 0..8 {
                let mask = 0x80 >> i;
                let bit = (mask & byte) >> (7 - i);

                if let (Some(left), Some(right)) = (codec_clone.root.left.clone(), codec_clone.root.right.clone()) {
                    codec_clone.root = if bit == 1 {
                        Box::leak(right).clone()
                    } else {
                        Box::leak(left).clone()
                    };
                    if codec_clone.root.left.is_none() {
                        if let Some(ch) = codec_clone.root.value {
                            output_string.push(ch);
                        }
                        codec_clone.root = tree.clone().root;
                    }
                }

                bit_counter += 1;
                if bit_counter == data_size {
                    break;
                }
            }
        }

        Ok(output_string)
    }
}

pub fn fill_code_map_outer(map: &mut BTreeMap<char, String>, tree: &HuffmanCodec) -> crate::Result<()> {
    fill_code_map_inner_recur(map, &tree.root, String::from(""))
}

pub fn fill_code_map_inner_recur(map: &mut BTreeMap<char, String>, tree: &HTree, prefix: String) -> crate::Result<()> {
    if tree.left.is_none() {
        let ch: char;
        if let Some(c) = tree.value {
            ch = c;
        } else {
            return Err(crate::Error::CompressionError("Error building the code map".into()));
        }
        map.insert(ch, prefix);
        return Ok(());
    }

    if let (Some(left), Some(right)) = (&tree.left, &tree.right) {
        fill_code_map_inner_recur(map, left, format!("{}0", prefix))?;
        fill_code_map_inner_recur(map, right, format!("{}1", prefix))?;
    }

    Ok(())
}

pub fn frequency_map(val: &str) -> BTreeMap<char, u32> {
    let mut out: BTreeMap<char, u32> = BTreeMap::new();

    val.chars().for_each(|ch| {
        let new = if let Some(o) = out.get(&ch) { o + 1u32 } else { 1u32 };
        out.insert(ch, new);
    });

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

    #[test]
    fn test_encode_decode() {
        let expected = "a";

        let compressed = HuffmanCodec::compress(expected.into()).unwrap();

        let decompressed = HuffmanCodec::decompress(&compressed).unwrap();

        assert_eq!(expected, decompressed);
    }

    // proptest! {
    //     #[test]
    //     fn prop_check_encode_decode(s in "[a-zA-Z0-9._!~$&'()*+;,=/?:@-]+[a-zA-Z0-9._!~$&'()*+;,=/?:@-]+") {
    //         let expected = String::from(&s);

    //         let compressed = HuffmanCodec::compress(s).unwrap();
    //         let decompressed = HuffmanCodec::decompress(&compressed).unwrap();

    //         assert_eq!(expected, decompressed);

    //     }

    //     #[test]
    //     fn prop_test_encode_decode_len(s in "[a-zA-Z0-9._!~$&'()*+;,=/?:@-]+[a-zA-Z0-9._!~$&'()*+;,=/?:@-]+") {
    //         let expected = s.len();

    //         let compressed = HuffmanCodec::compress(s).unwrap();
    //         let decompressed = HuffmanCodec::decompress(&compressed).unwrap();

    //         assert_eq!(expected, decompressed.len());
    //     }
    // }
}
