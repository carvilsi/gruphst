use gruphst::{attributes::Attributes, QueryAttribute, RUDAttribute};

fn prepare_attribute_test() -> (Attributes, String) {
    let mut attributes = Attributes::new();
    attributes.set("name", "foo");
    attributes.set("var", "bar");
    attributes.set("val", 2);
    (attributes.clone(), attributes.get_id())
}

#[test]
fn get_attribute() {
    let (attr, _id) = prepare_attribute_test();
    assert_eq!(attr.get("name").unwrap(), "foo");
    assert_eq!(attr.get("var").unwrap(), "bar");
}

#[test]
fn get_id() {
    let (attr, id) = prepare_attribute_test();
    assert_eq!(attr.get_id(), id);
}

#[test]
fn set_attribute() {
    let (mut attr, _id) = prepare_attribute_test();
    assert_eq!(attr.get("name").unwrap(), "foo");
    assert_eq!(attr.get("var").unwrap(), "bar");
    attr.set("animal", "pigeon");
    assert_eq!(attr.get("animal").unwrap(), "pigeon");
}

#[test]
fn update_attribute() {
    let (mut attr, _id) = prepare_attribute_test();
    assert_eq!(attr.get("name").unwrap(), "foo");
    assert_eq!(attr.get("var").unwrap(), "bar");
    attr.update("name", "lol").unwrap();
    assert_eq!(attr.get("name").unwrap(), "lol");
}

#[test]
fn update_attribute_fail() {
    let (mut attr, _id) = prepare_attribute_test();
    assert!(attr.update("foo", "lol").is_err());
}

#[test]
fn upsert_attribute() {
    let (mut attr, _id) = prepare_attribute_test();
    assert_eq!(attr.get("name").unwrap(), "foo");
    assert_eq!(attr.get("var").unwrap(), "bar");
    attr.upsert("name", "lol");
    assert_eq!(attr.get("name").unwrap(), "lol");
    attr.upsert("foo", "lol");
    assert_eq!(attr.get("foo").unwrap(), "lol");
}

#[test]
fn del_attribute() {
    let (mut attr, _id) = prepare_attribute_test();
    assert_eq!(attr.get("name").unwrap(), "foo");
    assert_eq!(attr.get("var").unwrap(), "bar");
    attr.delete("var").unwrap();
    assert!(attr.get("var").is_err());
    assert_eq!(attr.get("name").unwrap(), "foo");
}

#[test]
fn attribute_keys() {
    let (attr, _id) = prepare_attribute_test();
    let keys = attr.get_keys();
    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&&"name"));
    assert!(keys.contains(&&"val"));
    assert!(keys.contains(&&"var"));
    assert!(!keys.contains(&&"foo"));
}

#[test]
fn has_attribute() {
    let (attr, _id) = prepare_attribute_test();
    assert!(attr.has("name"));
    assert!(!attr.has("foobar"));
}

#[test]
fn like_attribute() {
    let (attr, _id) = prepare_attribute_test();
    assert!(attr.like("na"));
    assert!(attr.like("va"));
    assert!(!attr.like("fo"));
}

#[test]
fn equals_attr() {
    let (attr, _id) = prepare_attribute_test();
    assert!(attr.equals_to("name", "foo"));
    assert!(attr.equals_to("val", 2));
    assert!(!attr.equals_to("name", "fo"));
    assert!(!attr.equals_to("val", 3));
}

#[test]
fn length_attributes() {
    let (attr, _id) = prepare_attribute_test();
    assert_eq!(attr.len(), 3);
}

#[test]
fn is_empty_attributes() {
    let (attr, _id) = prepare_attribute_test();
    assert!(!attr.is_empty());
    let empty_attr = Attributes::new();
    assert!(empty_attr.is_empty());
}
