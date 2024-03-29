use bincode;
use crypto::{digest::Digest, sha3::Sha3};
use serde::{Deserialize, Serialize};

pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
    bincode::serialize(value).unwrap()
}

pub fn my_deserialize<'a, T: ?Sized>(bytes: &'a [u8]) -> T
where
    T: Deserialize<'a>,
{
    bincode::deserialize(bytes).unwrap()
}

pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use crate::coder::get_hash;

    use super::{my_deserialize, my_serialize, Point};

    #[test]
    fn test_serialize() {
        let point = Point { x: 1, y: 1 };
        let se = my_serialize(&point);
        let de: Point = my_deserialize(&se);
        assert_eq!(de, point);
    }

    #[test]
    fn test_hash() {
        let point = Point { x: 2, y: 2 };
        let se = my_serialize(&point);
        println!("{:?}", se);

        let res = get_hash(&se);
        println!("hash: {res}");
    }
}
