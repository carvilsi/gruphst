use gruphst::{
    attributes::Attributes,
    QueryAttribute,
    RUDAttribute
};

fn prepare_attribute_test() -> (Attributes, String) {
    let mut attributes = Attributes::new();
    attributes.set_attr("name", "foo");
    attributes.set_attr("var", "bar");
    attributes.set_attr("val", 2);
    (attributes.clone(), attributes.get_id())
}

#[test]
fn get_attribute() {
    let (attr, _id) = prepare_attribute_test();
    assert_eq!(attr.get_attr("name").unwrap(), "foo");
    assert_eq!(attr.get_attr("var").unwrap(), "bar");
}

#[test]
fn get_id() {
    let (attr, id) = prepare_attribute_test();
    assert_eq!(attr.get_id(), id);
}

#[test]
fn set_attribute() {
    let (mut attr, _id) = prepare_attribute_test();
    assert_eq!(attr.get_attr("name").unwrap(), "foo");
    assert_eq!(attr.get_attr("var").unwrap(), "bar");
    attr.set_attr("animal", "pigeon");
    assert_eq!(attr.get_attr("animal").unwrap(), "pigeon");
}

#[test]
fn update_attribute() {
    let (mut attr, _id) = prepare_attribute_test();
    assert_eq!(attr.get_attr("name").unwrap(), "foo");
    assert_eq!(attr.get_attr("var").unwrap(), "bar");
    attr.update_attr("name", "lol").unwrap();
    assert_eq!(attr.get_attr("name").unwrap(), "lol");
}

#[test]
fn update_attribute_fail() {
    let (mut attr, _id) = prepare_attribute_test();
    assert!(attr.update_attr("foo", "lol").is_err());
}

#[test]
fn upsert_attribute() {
    let (mut attr, _id) = prepare_attribute_test();
    assert_eq!(attr.get_attr("name").unwrap(), "foo");
    assert_eq!(attr.get_attr("var").unwrap(), "bar");
    attr.upsert_attr("name", "lol");
    assert_eq!(attr.get_attr("name").unwrap(), "lol");
    attr.upsert_attr("foo", "lol");
    assert_eq!(attr.get_attr("foo").unwrap(), "lol");
}

#[test]
fn del_attribute() {
    let (mut attr, _id) = prepare_attribute_test();
    assert_eq!(attr.get_attr("name").unwrap(), "foo");
    assert_eq!(attr.get_attr("var").unwrap(), "bar");
    attr.del_attr("var").unwrap();
    assert!(attr.get_attr("var").is_err());
    assert_eq!(attr.get_attr("name").unwrap(), "foo");
}

#[test]
fn attribute_keys() {
    let (attr, _id) = prepare_attribute_test();
    let keys = attr.get_attr_keys();
    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&&"name"));
    assert!(keys.contains(&&"val"));
    assert!(keys.contains(&&"var"));
    assert!(!keys.contains(&&"foo"));
}

#[test]
fn has_attribute() {
    let (attr, _id) = prepare_attribute_test();
    assert!(attr.has_attr("name"));
    assert!(!attr.has_attr("foobar"));
}

#[test]
fn like_attribute() {
    let (attr, _id) = prepare_attribute_test();
    assert!(attr.like_attr("na"));
    assert!(attr.like_attr("va"));
    assert!(!attr.like_attr("fo"));
}

#[test]
fn equals_attr() {
    let (attr, _id) = prepare_attribute_test();
    assert!(attr.equals_attr("name", "foo"));
    assert!(attr.equals_attr("val", 2));
    assert!(!attr.equals_attr("name", "fo"));
    assert!(!attr.equals_attr("val", 3));
}

#[test]
fn length_attributes() {
    let (attr, _id) = prepare_attribute_test();
    assert_eq!(attr.len_attr(), 3);
}

#[test]
fn is_empty_attributes() {
    let (attr, _id) = prepare_attribute_test();
    assert!(!attr.is_empty_attr());
    let empty_attr = Attributes::new();
    assert!(empty_attr.is_empty_attr());
}
