use gruphst::{attributes::Attributes, RUDAttribute};

fn prepare_attribute_test() -> (Attributes, String) {
    let mut attributes = Attributes::new();
    attributes.set_attr("name", "foo");
    attributes.set_attr("var", "bar");
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