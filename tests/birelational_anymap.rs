use std::any::TypeId;

use sicht::birelational_map::anyvec_map::BirelationalAnyValueMap;

type TestMap = BirelationalAnyValueMap<usize, usize, TypeId>;

#[test]
pub fn inserting() {
    let mut map = TestMap::new();
    map.insert(10usize, 20i32);

    let value = *map.get::<i32>(10usize).unwrap()[0];
    assert_eq!(value, 20);

    let key = *map.get_value(20i32).unwrap()[0];
    assert_eq!(key, 10);
}

#[test]
pub fn inserting_twice() {
    let mut map = TestMap::new();
    map.insert(10usize, 20i32);
    map.insert(10usize, 30i32);

    let value = map.get::<i32>(10usize).unwrap();
    assert_eq!(*value[0], 20);
    assert_eq!(*value[1], 30);

    let key = *map.get_value(30i32).unwrap()[0];
    assert_eq!(key, 10);

    let key = *map.get_value(20i32).unwrap()[0];
    assert_eq!(key, 10);
}

#[test]
pub fn removal() {
    let mut map = TestMap::new();
    map.insert(10usize, 20i32);
    map.insert(10usize, 30i32);

    map.remove(10usize, 30i32);

    let values = map.get::<i32>(10usize).unwrap();
    assert_eq!(values.len(), 1);
    assert_eq!(*values[0], 20);
}

#[test]
pub fn different_types() {
    let mut map = TestMap::new();
    map.insert(10usize, 20usize);
    map.insert(10usize, "hello, world");
    map.insert(10usize, 20i32);

    let value = map.get::<&str>(10usize).unwrap();
    assert_eq!(*value[0], "hello, world");

    let value = map.get::<i32>(10usize).unwrap();
    assert_eq!(*value[0], 20);

    let value = map.get::<usize>(10usize).unwrap();
    assert_eq!(*value[0], 20);

    let value = map.get_value("hello, world").unwrap();
    assert_eq!(*value[0], 10);

    let value = map.get_value(20usize).unwrap();
    assert_eq!(*value[0], 10);

    let value = map.get_value(20i32).unwrap();
    assert_eq!(*value[0], 10);
}

#[test]
pub fn different_types_multiple() {
    let mut map = TestMap::new();
    map.insert(10usize, "hello, world");
    map.insert(10usize, "world, hello");
    map.insert(10usize, 20i32);
    map.insert(10usize, 30i32);

    let value = map.get::<&str>(10usize).unwrap();
    assert_eq!(*value[0], "hello, world");
    assert_eq!(*value[1], "world, hello");

    let value = map.get::<i32>(10usize).unwrap();
    assert_eq!(*value[0], 20);
    assert_eq!(*value[1], 30);
}

#[test]
pub fn mutable_typed_lookup() {
    let mut map = TestMap::new();
    map.insert(10usize, 20i32);
    map.insert(10usize, 30i32);
    map.insert(10usize, "hello, world");

    let mut values = map.get_by_id_mut::<i32>(10usize).unwrap();
    *values[0] = 25;
    *values[1] = 35;
    drop(values);

    let values = map.get::<i32>(10usize).unwrap();
    assert_eq!(*values[0], 25);
    assert_eq!(*values[1], 35);

    let mut strings = map.get_mut::<&str>(10usize).unwrap();
    *strings[0] = "changed";
    drop(strings);

    let strings = map.get::<&str>(10usize).unwrap();
    assert_eq!(*strings[0], "changed");
}

#[test]
pub fn mutable_keys_and_values_of() {
    let mut map = TestMap::new();
    map.insert(10usize, 20i32);
    map.insert(11usize, 30i32);

    for value in map.values_of_mut::<i32>() {
        *value += 1;
    }

    let values = map.values_of::<i32>();
    assert_eq!(*values[0], 21);
    assert_eq!(*values[1], 31);

    for key in map.keys_mut() {
        *key += 100;
    }

    let keys = map.keys();
    assert_eq!(*keys[0], 110);
    assert_eq!(*keys[1], 111);
}
