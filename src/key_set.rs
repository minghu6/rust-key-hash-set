#![allow(dead_code)]

use std::collections::hash_map::{ HashMap };
use std::collections::hash_map;
use std::hash::{ Hash };
use std::iter:: { Map };
use std::fmt;
use std::vec;

pub type GetKeyType<T, K> = fn(&T) -> K;
pub type Map2SetType<T, K> = fn((K, T)) -> T;


////////////////////////////////////////////////////////////////////////////////
/// KeySet

pub trait KeySet <T, K> {
    /**
    * Create KeySet
    */
    fn new(get_key: GetKeyType<T, K>) -> Self;

    /**
    * Operate KeySet elem
    */
    fn insert(&mut self, value: T);
    fn contains(&self, value: &T) -> bool;
    fn remove(&mut self, value: &T) -> bool;
    fn take(&mut self, value: &T) -> Option<T>;
    fn get(&mut self, value: &T) -> Option<&T>;
    fn len(&self) -> usize;
    fn iter(&self) -> vec::IntoIter<&T>;

    /**
    * Check KeySet relationship
    */
    fn is_subset(&self, other: &Self) -> bool {
        self.iter().all(|x| other.contains(&x))
    }

    fn is_superset(&self, other: &Self) -> bool {
        other.iter().all(|x| self.contains(&x))
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        other.iter().all(|x| !self.contains(x))
    }

    /**
    * Operate with other KeySet
    */
    fn intersection<'a>(&'a self, other: &'a Self) -> Self;
    fn union<'a>(&'a self, other: &'a Self) -> Self;
    fn difference<'a>(&'a self, other: &'a Self) -> Self;
    fn symmetric_difference<'a>(&'a self, other: &'a Self) -> Self;
}


////////////////////////////////////////////////////////////////////////////////
/// Utils

pub fn debug_key<T: fmt::Debug>(value: &T) -> String {
    format!("{:?}", value)
}


////////////////////////////////////////////////////////////////////////////////
/// KeyHashSet

pub struct KeyHashSet<T, K: Hash> {
    get_key: GetKeyType<T, K>,
    _value_map: HashMap<K, T>,
}

impl <T, K> KeySet<T, K> for KeyHashSet<T, K> where T: Clone, K: Eq + Hash {
    fn new(get_key: GetKeyType<T, K>) -> Self {
        let _value_map:HashMap<K, T> = HashMap::new();

        KeyHashSet {
            get_key,
            _value_map,
        }
    }

    fn insert(&mut self, value:T) {
        let key = (self.get_key)(&value);

        self._value_map.insert(key, value);
    }

    fn contains(&self, value: &T) -> bool {
        let key = &(self.get_key)(value);

        self._value_map.contains_key(key)
    }

    // Rust doesn't open the constructor method for struct Draw
    // IndexHashMap drain range
    // pub fn drain(&mut self) -> Map<HashMap::map::Drain<'_, K, T>, Map2SetType<T, K>> {
    //     self._value_map.drain().map(|(_, v)| v)
    // }

    fn remove(&mut self, value:&T) -> bool {
        let key = &(self.get_key)(value);

        match self._value_map.remove(key) {
            None => false,
            _ => true
        }
    }

    fn take(&mut self, value:&T) -> Option<T> {
        let key = &(self.get_key)(value);

        self._value_map.remove(key)
    }

    fn get(&mut self, value:&T) -> Option<&T> {
        let key = &(self.get_key)(value);

        self._value_map.get(key)
    }

    fn len(&self) -> usize {
        return self._value_map.len();
    }

    fn iter(&self) -> vec::IntoIter<&T> {
        let res: Vec<&T> = self._value_map.values ().collect();
        res.into_iter()
    }

    fn intersection<'a>(&'a self, other: &'a Self) -> Self {
        let mut new_set = KeyHashSet::new(self.get_key);
        for v in self.iter().chain(other.iter()) {
            new_set.insert(v.clone())
        }

        new_set
    }

    fn union<'a>(&'a self, other: &'a Self) -> Self {
        let mut new_set = KeyHashSet::new(self.get_key);

        for v in self.iter().filter(|v| other.contains(v)) {
            new_set.insert(v.clone())
        }

        new_set
    }

    fn difference<'a>(&'a self, other: &'a Self) -> Self {
        let mut new_set = KeyHashSet::new(self.get_key);

        for v in self.iter().filter(|v| !other.contains(v)) {
            new_set.insert(v.clone())
        }

        new_set
    }

    fn symmetric_difference<'a>(&'a self, other: &'a Self) -> Self {
        let mut new_set = KeyHashSet::new(self.get_key);

        for v in self.iter().filter(|v| !other.contains(v)) {
            new_set.insert(v.clone())
        }

        for v in other.iter().filter(|v| !self.contains(v)) {
            new_set.insert(v.clone())
        }

        new_set
    }
}

/// IntoIterator for KeyHashSet
impl<T, K> IntoIterator for KeyHashSet<T, K> where K: Hash {
    type Item = T;
    type IntoIter = Map<hash_map::IntoIter<K, T>, Map2SetType<T, K>>;

    fn into_iter(self) -> Self::IntoIter {
        self._value_map.into_iter().map(|(_, v)| v)
    }
}

impl<T, K> PartialEq for KeyHashSet<T, K> where T: Clone, K: Eq + Hash {
    fn eq(&self, other: &Self) -> bool {
        self.is_subset(other) && other.is_subset(self)
    }
}

impl<T, K> fmt::Debug for KeyHashSet<T, K> where T: Clone + fmt::Debug, K: fmt::Debug + Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyHashSet")
         .field("_value_map", &self._value_map)
         .finish()
    }
}

/// Just for hide abstraction
pub struct IteratorWrapper<I, T> where I: Iterator<Item=T> {
    iter: I,
}

impl<I, T> IteratorWrapper<I, T> where I: Iterator<Item=T> {
    pub fn new(iter: I) -> IteratorWrapper<I, T> where I: Iterator {
        IteratorWrapper {
            iter,
        }
    }
}

impl<I, T> Iterator for IteratorWrapper<I, T>  where I: Iterator<Item=T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
}