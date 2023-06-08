use std::collections::{HashMap, BTreeMap};
use std::hash::Hash;
use std::sync::{Arc, RwLock};

pub trait DeepClone {
    fn deep_clone(&self) -> Self;
}

impl<T> DeepClone for Arc<RwLock<T>> where T: DeepClone {
    fn deep_clone(&self) -> Self {
        return Arc::new(RwLock::new(self.read().unwrap().deep_clone()));
    }
}

impl DeepClone for String {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}

impl DeepClone for usize {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}

impl DeepClone for bool {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}

impl<T> DeepClone for Vec<T> where T: DeepClone {
    fn deep_clone(&self) -> Self {
        let mut output = vec![];
        for target in self {
            output.push(target.deep_clone());
        }
        output
    }
}

impl<T> DeepClone for Option<T> where T: DeepClone {
    fn deep_clone(&self) -> Self {
        return match self {
            None => { None }
            Some(target) => { Some(target.deep_clone()) }
        }
    }
}

impl <K,V> DeepClone for HashMap<K,V> where K : Eq + Hash + DeepClone, V: DeepClone {
    fn deep_clone(&self) -> Self {
        let mut output = HashMap::new();
        for (name, v) in self {
            output.insert(name.deep_clone(), v.deep_clone());
        }
        output
    }
}

impl <K,V> DeepClone for BTreeMap<K,V> where K : Eq + std::cmp::Ord + DeepClone, V: DeepClone {
    fn deep_clone(&self) -> Self {
        let mut output = BTreeMap::new();
        for (name, v) in self {
            output.insert(name.deep_clone(), v.deep_clone());
        }
        output
    }
}

#[allow(non_camel_case_types)]
pub trait DeepClone_ArcLock {
    fn deep_clone_arclock(&self) -> Arc<RwLock<Self>>;
}
