#![allow(dead_code)]

use std::collections::{ HashMap };
use std::collections::hash_map;
use std::hash::{ Hash };
use std::iter:: { Map };
use std::fmt;

pub type GetKeyType<T> = fn(&T) -> String;
pub type Map2SetType<T> = fn((String, T)) -> T;

pub struct KeySet<T> {
    get_key: GetKeyType<T>,
    _value_map: HashMap<String, T>,
}

pub fn debug_key<T: fmt::Debug>(value: &T) -> String {
    format!("{:?}", value)
}

impl<T> KeySet<T> where T: Clone {
    pub fn new(get_key: GetKeyType<T>) -> Self {
        let _value_map:HashMap<String, T> = HashMap::new();

        KeySet {
            get_key,
            _value_map,
        }
    }

    pub fn insert(&mut self, value:T) {
        let key = (self.get_key)(&value);

        self._value_map.insert(key, value);
    }

    pub fn contains(&self, value: &T) -> bool {
        let key = &(self.get_key)(value);

        self._value_map.contains_key(key)
    }

    // Rust doesn't open the constructor method for struct Draw
    pub fn drain(&mut self) -> IteratorWrapper<Map<hash_map::Drain<String, T>, Map2SetType<T>>, T> {
        IteratorWrapper::new(self._value_map.drain().map(|(_, v)| v))
    }

    pub fn remove(&mut self, value:&T) -> bool {
        let key = &(self.get_key)(value);

        match self._value_map.remove(key) {
            None => false,
            _ => true
        }
    }

    pub fn take(&mut self, value:&T) -> Option<T> {
        let key = &(self.get_key)(value);

        self._value_map.remove(key)
    }

    pub fn get(&mut self, value:&T) -> Option<&T> {
        let key = &(self.get_key)(value);

        self._value_map.get(key)
    }

    pub fn len(&self) -> usize {
        return self._value_map.len();
    }

    pub fn iter(&self) -> hash_map::Values<String, T> {
        self._value_map.values()
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.iter().all(|x| other.contains(&x))
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        other.iter().all(|x| self.contains(&x))
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.union(other).is_empty()
    }

    pub fn intersection<'a>(&'a self, other: &'a Self) -> Self {
        let mut new_set = KeySet::new(self.get_key);
        for v in self.iter().chain(other.iter()) {
            new_set.insert(v.clone())
        }

        new_set
    }

    pub fn union<'a>(&'a self, other: &'a Self) -> Self {
        let mut new_set = KeySet::new(self.get_key);

        for v in self.iter().filter(|v| other.contains(v)) {
            new_set.insert(v.clone())
        }

        new_set
    }

    pub fn difference<'a>(&'a self, other: &'a Self) -> Self {
        let mut new_set = KeySet::new(self.get_key);

        for v in self.iter().filter(|v| !other.contains(v)) {
            new_set.insert(v.clone())
        }

        new_set
    }

    pub fn symmetric_difference<'a>(&'a self, other: &'a Self) -> Self {
        let mut new_set = KeySet::new(self.get_key);

        for v in self.iter().filter(|v| !other.contains(v)) {
            new_set.insert(v.clone())
        }

        for v in other.iter().filter(|v| !self.contains(v)) {
            new_set.insert(v.clone())
        }

        new_set
    }
}


impl<T> IntoIterator for KeySet<T> {
    type Item = T;
    type IntoIter = Map<hash_map::IntoIter<String, T>, Map2SetType<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self._value_map.into_iter().map(|(_, v)| v)
    }
}

impl<T> PartialEq for KeySet<T> where T: Hash + Clone {
    fn eq(&self, other: &Self) -> bool {
        self.is_subset(other) && other.is_subset(self)
    }
}

impl<T> fmt::Debug for KeySet<T> where T: fmt::Debug {
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