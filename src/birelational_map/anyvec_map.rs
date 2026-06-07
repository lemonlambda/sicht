use slotmap::{DefaultKey, SlotMap};
use std::{any::Any, collections::HashMap, hash::Hash};

use crate::birelational_map::BirelationalId;

pub struct BirelationalAnyValueMap<K, KId, VId>
where
    K: BirelationalId<KId>,
    VId: Hash + Eq + PartialEq,
    KId: Hash + Eq + PartialEq,
{
    keys: SlotMap<DefaultKey, K>,
    values: SlotMap<DefaultKey, Box<dyn Any>>,
    keys_map: HashMap<KId, (DefaultKey, Vec<DefaultKey>)>,
    values_map: HashMap<VId, (DefaultKey, Vec<(DefaultKey, DefaultKey)>)>,
}

impl<K, KId, VId> BirelationalAnyValueMap<K, KId, VId>
where
    K: BirelationalId<KId>,
    VId: Hash + Eq + PartialEq,
    KId: Hash + Eq + PartialEq,
{
    pub fn new() -> Self {
        Self {
            keys: SlotMap::new(),
            values: SlotMap::new(),
            keys_map: HashMap::new(),
            values_map: HashMap::new(),
        }
    }

    pub fn get<V>(&mut self, key: K) -> Option<Vec<&'_ V>>
    where
        V: BirelationalId<VId> + 'static,
    {
        self.keys_map
            .get(&key.get_id())?
            .1
            .iter()
            .filter_map(|x| self.values.get(*x)?.downcast_ref::<V>())
            .collect::<Vec<_>>()
            .into()
    }

    pub fn get_value<V>(&mut self, value: V) -> Option<Vec<&'_ K>>
    where
        V: BirelationalId<VId> + 'static,
        for<'a> &'a V: PartialEq,
    {
        self.values_map
            .get(&value.get_id())?
            .1
            .iter()
            .filter(|(_, value_idx)| {
                self.values
                    .get(*value_idx)
                    .and_then(|value| value.downcast_ref())
                    == Some(&value)
            })
            .map(|(key_idx, _)| self.keys.get(*key_idx))
            .collect()
    }

    pub fn insert_boxed_ref(&mut self, key: &K, value_id: VId, value: Box<dyn Any>) {
        let (key_id, value_id) = (key.get_id(), value_id);
        let key_exists = self.keys_map.contains_key(&key_id);
        let value_exists = self.values_map.contains_key(&value_id);

        match (key_exists, value_exists) {
            (true, true) => {
                let key_idx = self.keys_map.get(&key_id).unwrap().0;
                let value_idx = self.values.insert(value);

                let (_, values) = self.keys_map.get_mut(&key_id).unwrap();
                values.push(value_idx);

                let (_, relations) = self.values_map.get_mut(&value_id).unwrap();
                relations.push((key_idx, value_idx));
            }
            (true, false) => {
                let key_idx = self.keys_map.get(&key_id).unwrap().0;
                let value_idx = self.values.insert(value);

                let (_, values) = self.keys_map.get_mut(&key_id).unwrap();
                values.push(value_idx);
                self.values_map
                    .insert(value_id, (value_idx, vec![(key_idx, value_idx)]));
            }
            _ => panic!("Key must exist before being able to use it as ref."),
        }
    }

    pub fn insert_boxed(&mut self, key: K, value_id: VId, value: Box<dyn Any>) {
        let (key_id, value_id) = (key.get_id(), value_id);
        let key_exists = self.keys_map.contains_key(&key_id);
        let value_exists = self.values_map.contains_key(&value_id);

        match (key_exists, value_exists) {
            (false, true) => {
                let value_idx = self.values.insert(value);
                let key_idx = self.keys.insert(key);

                self.keys_map.insert(key_id, (key_idx, vec![value_idx]));
                let (_, relations) = self.values_map.get_mut(&value_id).unwrap();
                relations.push((key_idx, value_idx));
            }
            (false, false) => {
                let key_idx = self.keys.insert(key);
                let value_idx = self.values.insert(value);

                self.keys_map.insert(key_id, (key_idx, vec![value_idx]));
                self.values_map
                    .insert(value_id, (value_idx, vec![(key_idx, value_idx)]));
            }
            _ => self.insert_boxed_ref(&key, value_id, value),
        }
    }

    pub fn insert_ref<V>(&mut self, key: &K, value: V)
    where
        V: BirelationalId<VId> + 'static,
    {
        self.insert_boxed_ref(key, value.get_id(), Box::new(value));
    }

    pub fn insert<V>(&mut self, key: K, value: V)
    where
        V: BirelationalId<VId> + 'static,
    {
        self.insert_boxed(key, value.get_id(), Box::new(value));
    }

    pub fn remove<V>(&mut self, key: K, value: V)
    where
        V: BirelationalId<VId> + 'static,
        for<'a> &'a V: PartialEq,
        for<'a> &'a K: PartialEq,
    {
        if let Some((_, items)) = self.keys_map.get_mut(&key.get_id()) {
            let taken_items = std::mem::take(items);
            *items = taken_items
                .into_iter()
                .filter(|x| {
                    let x = self.values.get(*x).and_then(|x| x.downcast_ref());
                    x != Some(&value)
                })
                .collect();
        }

        if let Some((_, items)) = self.values_map.get_mut(&value.get_id()) {
            let taken_items = std::mem::take(items);
            *items = taken_items
                .into_iter()
                .filter(|(key_idx, value_idx)| {
                    self.keys.get(*key_idx) != Some(&key)
                        || self
                            .values
                            .get(*value_idx)
                            .and_then(|value| value.downcast_ref())
                            != Some(&value)
                })
                .collect();
        }
    }
}

impl<K, KId, VId> Default for BirelationalAnyValueMap<K, KId, VId>
where
    K: BirelationalId<KId>,
    VId: Hash + Eq + PartialEq,
    KId: Hash + Eq + PartialEq,
{
    fn default() -> Self {
        Self::new()
    }
}
