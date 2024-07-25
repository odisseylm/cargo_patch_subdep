use std::collections::{ HashMap, HashSet };
use std::hash::Hash;
//--------------------------------------------------------------------------------------------------


#[inline]
#[allow(dead_code)]
pub fn hash_map_1<K: Hash + Eq, V>(k: K, v: V) -> HashMap<K, V> {
    let mut map = HashMap::<K,V>::new();
    map.insert(k, v);
    map
}
#[inline]
#[allow(dead_code)]
pub fn hash_map<const N: usize, K: Hash + Eq, V>(entries: [(K, V); N]) -> HashMap<K, V> {
    let mut map = HashMap::new();
    for e in entries {
        map.insert(e.0, e.1);
    }
    map
}

#[inline]
#[allow(dead_code)]
pub fn string_hash_map_1(k: &str, v: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert(k.to_owned(), v.to_owned());
    map
}
#[inline]
#[allow(dead_code)]
pub fn string_hash_map<const N: usize>(entries: [(&str, &str);N]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for e in entries {
        map.insert(e.0.to_owned(), e.1.to_owned());
    }
    map
}


#[inline]
#[allow(dead_code)]
pub fn hash_set_1<V: Hash + Eq>(v: V) -> HashSet<V> {
    let mut set = HashSet::new();
    set.insert(v);
    set
}
#[inline]
#[allow(dead_code)]
pub fn string_hash_set_1(v: &str) -> HashSet<String> {
    let mut set = HashSet::new();
    set.insert(v.to_owned());
    set
}
#[inline]
#[allow(dead_code)]
pub fn string_hash_set<const N: usize>(values: [&str;N]) -> HashSet<String> {
    let mut set = HashSet::<String>::new();
    for v in values {
        set.insert((*v).to_owned());
    }
    set
}
