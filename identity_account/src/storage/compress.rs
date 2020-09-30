use std::{
    cmp::Ordering,
    collections::{binary_heap::BinaryHeap, BTreeMap},
    rc::Rc,
    string::String,
    vec::Vec,
};

pub struct Codec(pub BTreeMap<char, Vec<u8>>);

#[derive(Eq, Debug, Clone)]
struct HTree {
    count: i32,
    value: Option<char>,
    left: Option<Rc<HTree>>,
    right: Option<Rc<HTree>>,
}

impl Codec {
    pub fn new(s: &str) -> crate::Result<Self> {
        let f_map = frequency_map(s);
        let heap = map_to_heap(f_map);
        let tree = heap_to_tree(heap)?;

        Ok(Self(tree_to_codes(&Some(tree), Vec::new(), BTreeMap::new())))
    }

    pub fn encode(&self, data: String) -> crate::Result<Vec<u8>> {
        let mut num_bits = 0;

        data.chars()
            .map(|c| {
                if let Some(code) = self.0.get(&c) {
                    num_bits += code.len();
                    Ok(())
                } else {
                    Err(crate::Error::CompressionError("Doesn't exist in codec".into()))
                }
            })
            .collect::<crate::Result<()>>()?;

        let mut ret = Vec::<u8>::with_capacity(num_bits);

        data.chars().for_each(|ch| {
            let val = self.0.get(&ch).expect("Should exist but doesn't");

            val.iter().for_each(|bit| ret.push(*bit));
        });

        Ok(ret)
    }

    pub fn decode(&self, data: Vec<u8>) -> String {
        let code = reverse(&self.0);

        let mut temp = Vec::<u8>::new();
        let mut ret = String::new();

        data.into_iter().for_each(|b| {
            temp.push(b);

            if let Some(c) = code.get(&temp) {
                ret.push(*c);
                temp.clear();
            }
        });
        ret
    }
}

impl HTree {
    pub fn new(value: char, count: i32) -> Rc<Self> {
        Rc::new(HTree {
            count,
            value: Some(value),
            left: None,
            right: None,
        })
    }

    pub fn merge(tree_left: Rc<HTree>, tree_right: Rc<HTree>) -> Rc<HTree> {
        Rc::new(HTree {
            count: tree_left.count + tree_right.count,
            value: None,
            left: Some(tree_left),
            right: Some(tree_right),
        })
    }
}

fn map_to_heap(map: BTreeMap<char, i32>) -> BinaryHeap<Rc<HTree>> {
    let mut heap = BinaryHeap::new();
    map.into_iter().for_each(|(l, c)| {
        let t = HTree::new(l, c);
        heap.push(t);
    });

    heap
}

fn heap_to_tree(mut heap: BinaryHeap<Rc<HTree>>) -> crate::Result<Rc<HTree>> {
    while heap.len() > 1 {
        let (t1, t2) = (
            heap.pop()
                .ok_or_else(|| crate::Error::CompressionError("Error popping off of Heap".into()))?,
            heap.pop()
                .ok_or_else(|| crate::Error::CompressionError("Error popping off of Heap".into()))?,
        );

        heap.push(HTree::merge(t1, t2));
    }

    heap.pop()
        .ok_or_else(|| crate::Error::CompressionError("Error popping off of Heap".into()))
}

fn tree_to_codes(
    root: &Option<Rc<HTree>>,
    prefix: Vec<u8>,
    mut map: BTreeMap<char, Vec<u8>>,
) -> BTreeMap<char, Vec<u8>> {
    if let Some(ref tree) = *root {
        match tree.value {
            Some(t) => {
                map.insert(t, prefix);
            }
            None => {
                let (mut pre_l, mut pre_r) = (prefix.clone(), prefix);
                pre_l.push(1u8);
                let map = tree_to_codes(&tree.left, pre_l, map);
                pre_r.push(0u8);

                return tree_to_codes(&tree.right, pre_r, map);
            }
        }
    }

    map
}

fn reverse(tree: &BTreeMap<char, Vec<u8>>) -> BTreeMap<Vec<u8>, char> {
    let mut ret = BTreeMap::new();

    tree.iter().for_each(|(k, v)| {
        ret.insert(v.clone(), *k);
    });

    ret
}

fn frequency_map(n: &str) -> BTreeMap<char, i32> {
    let mut output: BTreeMap<char, i32> = BTreeMap::new();
    n.chars().for_each(|c| {
        let new = if let Some(o) = output.get(&c) { o + 1i32 } else { 1i32 };
        output.insert(c, new);
    });
    output
}

impl Ord for HTree {
    fn cmp(&self, other: &HTree) -> Ordering {
        (self.count).cmp(&(other.count))
    }
}

impl PartialOrd for HTree {
    fn partial_cmp(&self, other: &HTree) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HTree {
    fn eq(&self, other: &HTree) -> bool {
        self.count == other.count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_frequency() {
        let val = "abababababaaababbbbabababbabccbcccbabbdddddd";

        let res = frequency_map(val);
        let mut expected: BTreeMap<char, i32> = BTreeMap::new();

        expected.insert('a', 14);
        expected.insert('b', 19);
        expected.insert('c', 5);
        expected.insert('d', 6);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_decode() {
        let val = String::from("abababababaaababbbbabababbabccbcccbabbdddddd");
        let table = "abcdefghijklmnopqrstuvwxyz";
        let codec = Codec::new(table).unwrap();
        let encode = codec.encode(val.clone()).unwrap();

        let decode = codec.decode(encode);

        assert_eq!(val, decode);

        let val = String::from("oaiusdoiakjnckjhasd");

        let encode = codec.encode(val.clone()).unwrap();
        let decode = codec.decode(encode);

        assert_eq!(val, decode);

        let val = String::from("ABC");

        assert!(codec.encode(val).is_err());
    }
}
