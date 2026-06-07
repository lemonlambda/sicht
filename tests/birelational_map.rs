use sicht::birelational_map::BirelationalMap;

type TestMap = BirelationalMap<usize, usize, usize, usize>;

#[test]
pub fn inserting() {
    let mut map = TestMap::new();
    map.insert(10usize, 20usize);

    let value = *map.get(10usize).unwrap()[0];
    assert_eq!(value, 20);

    let key = *map.get_value(20usize).unwrap()[0];
    assert_eq!(key, 10);
}

#[test]
pub fn inserting_twice() {
    let mut map = TestMap::new();
    map.insert(10usize, 20usize);
    map.insert(10usize, 30usize);

    let value = map.get(10usize).unwrap();
    assert_eq!(*value[0], 20);
    assert_eq!(*value[1], 30);

    let key = *map.get_value(30usize).unwrap()[0];
    assert_eq!(key, 10);

    let key = *map.get_value(20usize).unwrap()[0];
    assert_eq!(key, 10);
}

#[test]
pub fn removal() {
    let mut map = TestMap::new();
    map.insert(10usize, 20usize);
    map.insert(10usize, 30usize);

    map.remove(10usize, 30usize);

    let values = map.get(10usize).unwrap();
    assert_eq!(values.len(), 1);
    assert_eq!(*values[0], 20);
}
