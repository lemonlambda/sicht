use slotmap::{DefaultKey, DenseSlotMap};

use crate::birelational_map::BirelationalId;

use std::{collections::HashMap, hash::Hash};

pub struct BirelationalMap<K, KId, V, VId>
where
    K: BirelationalId<KId>,
    V: BirelationalId<VId>,
    KId: Hash + Eq + PartialEq,
    VId: Hash + Eq + PartialEq,
{
    keys: DenseSlotMap<DefaultKey, K>,
    values: DenseSlotMap<DefaultKey, V>,
    keys_map: HashMap<KId, (DefaultKey, Vec<DefaultKey>)>,
    values_map: HashMap<VId, (DefaultKey, Vec<DefaultKey>)>,
}

impl<K, KId, V, VId> BirelationalMap<K, KId, V, VId>
where
    K: BirelationalId<KId>,
    V: BirelationalId<VId>,
    KId: Hash + Eq + PartialEq,
    VId: Hash + Eq + PartialEq,
{
    pub fn new() -> Self {
        Self {
            keys: DenseSlotMap::new(),
            values: DenseSlotMap::new(),
            keys_map: HashMap::new(),
            values_map: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: K) -> Option<Vec<&'_ V>> {
        self.keys_map
            .get(&key.get_id())?
            .1
            .iter()
            .map(|x| self.values.get(*x))
            .collect()
    }

    pub fn get_value(&mut self, value: V) -> Option<Vec<&'_ K>> {
        self.values_map
            .get(&value.get_id())?
            .1
            .iter()
            .map(|x| self.keys.get(*x))
            .collect()
    }

    pub fn insert(&mut self, key: K, value: V) {
        let (key_id, value_id) = (key.get_id(), value.get_id());
        let key_exists = self.keys_map.contains_key(&key_id);
        let value_exists = self.values_map.contains_key(&value_id);

        match (key_exists, value_exists) {
            (true, true) => {
                let key_idx = self.keys_map.get(&key_id).unwrap().0;
                let value_idx = self.values_map.get(&value_id).unwrap().0;

                let (_, values) = self.keys_map.get_mut(&key_id).unwrap();
                if !values.contains(&value_idx) {
                    values.push(value_idx);
                }

                let (_, keys) = self.values_map.get_mut(&value_id).unwrap();
                if !keys.contains(&key_idx) {
                    keys.push(key_idx);
                }
            }
            (true, false) => {
                let key_idx = self.keys_map.get(&key_id).unwrap().0;
                let value_idx = self.values.insert(value);

                let (_, values) = self.keys_map.get_mut(&key_id).unwrap();
                values.push(value_idx);
                self.values_map.insert(value_id, (value_idx, vec![key_idx]));
            }
            (false, true) => {
                let value_idx = self.values_map.get(&value_id).unwrap().0;
                let key_idx = self.keys.insert(key);

                self.keys_map.insert(key_id, (key_idx, vec![value_idx]));
                let (_, keys) = self.values_map.get_mut(&value_id).unwrap();
                keys.push(key_idx);
            }
            (false, false) => {
                let key_idx = self.keys.insert(key);
                let value_idx = self.values.insert(value);

                self.keys_map.insert(key_id, (key_idx, vec![value_idx]));
                self.values_map.insert(value_id, (value_idx, vec![key_idx]));
            }
        }
    }

    pub fn remove(&mut self, key: K, value: V)
    where
        for<'a> &'a V: PartialEq,
        for<'a> &'a K: PartialEq,
    {
        if let Some((_, items)) = self.keys_map.get_mut(&key.get_id()) {
            let taken_items = std::mem::take(items);
            *items = taken_items
                .into_iter()
                .filter(|x| self.values.get(*x) != Some(&value))
                .collect();
        }

        if let Some((_, items)) = self.values_map.get_mut(&value.get_id()) {
            let taken_items = std::mem::take(items);
            *items = taken_items
                .into_iter()
                .filter(|x| self.keys.get(*x) != Some(&key))
                .collect();
        }
    }
}

impl<K, KId, V, VId> Default for BirelationalMap<K, KId, V, VId>
where
    K: BirelationalId<KId>,
    V: BirelationalId<VId>,
    KId: Hash + Eq + PartialEq,
    VId: Hash + Eq + PartialEq,
{
    fn default() -> Self {
        Self::new()
    }
}
