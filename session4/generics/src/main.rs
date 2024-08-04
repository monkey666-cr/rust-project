use std::{
    collections::{btree_map::Values, HashMap},
    fmt::Debug,
};

fn just_print_it<T>(x: T)
where
    T: ToString + Debug,
{
    println!("{:#?}", x);
}

struct Point {
    x: i32,
    y: i32,
}

impl ToString for Point {
    fn to_string(&self) -> String {
        format!("Point.x: {}, Point:y: {}", self.x, self.y)
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

struct Degrees(f32);

impl From<Radians> for Degrees {
    fn from(rad: Radians) -> Self {
        Degrees(rad.0 * 180.0 / std::f32::consts::PI)
    }
}

struct Radians(f32);

impl From<Degrees> for Radians {
    fn from(deg: Degrees) -> Self {
        Radians(deg.0 * std::f32::consts::PI / 180.0)
    }
}

fn sin(angle: impl Into<Radians>) -> f32 {
    let angle: Radians = angle.into();
    angle.0.sin()
}

#[derive(Debug)]
struct HashMapBucket<K, V> {
    map: HashMap<K, Vec<V>>,
}

impl<K, V> HashMapBucket<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn new() -> Self {
        HashMapBucket {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: K, value: V) {
        let values = self.map.entry(key).or_insert(Vec::new());
        values.push(value);
    }
}

impl<K, V> HashMapBucket<K, V> {
    fn iter(&self) -> HashMapBucketIter<K, V> {
        let mut key_iter = self.map.iter();
        let current_map_entry = key_iter.next();

        HashMapBucketIter {
            key_iter,
            current_map_entry,
            current_vec_index: 0,
        }
    }
}

struct HashMapBucketIter<'a, K, V> {
    key_iter: std::collections::hash_map::Iter<'a, K, Vec<V>>,
    current_map_entry: Option<(&'a K, &'a Vec<V>)>,
    current_vec_index: usize,
}

impl<'a, K, V> Iterator for HashMapBucketIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((key, value)) = self.current_map_entry {
            if self.current_vec_index < value.len() {
                let value = &value[self.current_vec_index];
                self.current_vec_index += 1;
                return Some((key, value));
            } else {
                self.current_map_entry = self.key_iter.next();
                self.current_vec_index = 0;

                if let Some((key, value)) = self.current_map_entry {
                    if self.current_vec_index < value.len() {
                        let value = &value[self.current_vec_index];
                        self.current_vec_index += 1;
                        return Some((key, value));
                    }
                }
            }
        }

        None
    }
}

fn main() {
    let s = "hello".to_string();
    just_print_it(s);

    let p = Point { x: 1, y: 2 };
    just_print_it(p);

    let behind_you = Degrees(180.0);
    // sin(behind_you);

    let behind_you_radians = Radians::from(behind_you);
    let behind_you_radians2: Radians = Degrees(180.0).into();

    sin(behind_you_radians2);

    let mut h = HashMapBucket::new();
    h.insert("hello", 1);
    println!("{:?}", h);

    let mut h = HashMapBucket::new();
    h.insert("hello", "world");
    h.insert("hello1", "world");
    h.insert("hello2", "world");

    for (key, value) in h.iter() {
        println!("{key}, {value}");
    }
}
