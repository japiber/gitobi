use std::io;
use serde_json::{Map, Value};
use gitobi::json_document::{contains_key, delete_key, map_from_str, map_into_string, update_key, Document};
use gitobi::query::{QryClause, QueryClause, QueryableDocument};

#[test]
fn json_document_update_key() {
    let data = r#"
        {
            "name": "John",
            "age": 43,
            "address": {
                "zip": 7777,
                "city": "Edoras",
                "state": "Rohan"
            },
            "phones": {
                "office": "+44 1234567",
                "personal": "+44 2345678"
            }
        }"#;


    let doc = serde_json::from_str(data).unwrap();

    let updated_doc = update_key("address.zip", &doc, 666.into()).unwrap();
    let address = updated_doc.get("address").unwrap().as_object().unwrap();
    let zip = address.get("zip").unwrap().as_i64().unwrap();
    assert_eq!(zip, 666);

    let updated_doc = update_key("phones.work", &doc, "+44 123 456 789".into()).unwrap();
    let phones = updated_doc.get("phones").unwrap().as_object().unwrap();
    let work = phones.get("work").unwrap().as_str().unwrap();
    assert_eq!(work, "+44 123 456 789");

    let updated_doc = update_key("foo.waz.kaq", &doc, "+44 123 456 789".into()).unwrap();
    assert!(contains_key("foo.waz.kaq", &updated_doc));
    let foo = updated_doc.get("foo");
    assert!(foo.is_some());
    let waz = foo.unwrap().as_object().unwrap().get("waz");
    assert!(waz.is_some());
    let kaq = waz.unwrap().as_object().unwrap().get("kaq");
    assert!(kaq.is_some());
    assert_eq!(kaq.unwrap().as_str().unwrap(), "+44 123 456 789");

    assert!(contains_key("address.zip", &doc));
    assert!(!contains_key("address.foo", &doc));
}

#[test]
fn json_document_delete_key() {
    let data = r#"
        {
            "name": "John",
            "age": 43,
            "address": {
                "zip": 7777,
                "city": "Edoras",
                "state": "Rohan"
            },
            "phones": {
                "office": "+44 1234567",
                "personal": "+44 2345678"
            }
        }"#;


    let doc = serde_json::from_str(data).unwrap();

    let deleted_doc = delete_key("address.zip", &doc).unwrap();
    let address = deleted_doc.get("address").unwrap().as_object().unwrap();
    assert!(address.get("zip").is_none());

    let deleted_doc = delete_key("phones.office", &doc).unwrap();
    let phones = deleted_doc.get("phones").unwrap().as_object().unwrap();
    assert!(phones.get("office").is_none());
}

#[test]
fn json_document_contains_key() {
    let data = r#"
        {
            "name": "John",
            "age": 43,
            "address": {
                "zip": 7777,
                "city": "Edoras",
                "state": "Rohan"
            },
            "phones": {
                "office": "+44 1234567",
                "personal": "+44 2345678"
            }
        }"#;

    let doc = serde_json::from_str(data).unwrap();
    assert!(contains_key("address.zip", &doc));
    assert!(!contains_key("address.zip.foo", &doc));
}

#[test]
fn json_document_load() {
    let data = r#"
        {
            "name": "John",
            "age": 43,
            "address": {
                "zip": 7777,
                "city": "Edoras",
                "state": "Rohan"
            },
            "phones": {
                "office": "+44 1234567",
                "personal": "+44 2345678"
            }
        }"#;

    let map_doc: Map<String, Value> = serde_json::from_str(data).unwrap();

    let mut reader = io::BufReader::new(data.as_bytes());

    let doc = Document::load(&mut reader, map_from_str).unwrap();

    assert_eq!(*doc.content(), map_doc);
}

#[test]
fn json_document_write() {
    let data = r#"
        {
            "name": "John",
            "age": 43,
            "address": {
                "zip": 7777,
                "city": "Edoras",
                "state": "Rohan"
            },
            "phones": {
                "office": "+44 1234567",
                "personal": "+44 2345678"
            }
        }"#;

    let map_doc: Map<String, Value> = serde_json::from_str(data).unwrap();

    let mut writer = Vec::new();
    let mut reader = io::BufReader::new(data.as_bytes());


    let mut doc = Document::load(&mut reader, map_from_str).unwrap();
    doc.write(&mut writer, map_into_string).expect("write panic message");

    let write_data = str::from_utf8(writer.as_slice()).unwrap();
    let map_write: Map<String, Value> = serde_json::from_str(write_data).unwrap();

    assert_eq!(map_doc, map_write);
}

#[test]
fn json_document_update() {
    let data = r#"
        {
            "name": "John",
            "age": 43,
            "address": {
                "zip": 7777,
                "city": "Edoras",
                "state": "Rohan"
            },
            "phones": {
                "office": "+44 1234567",
                "personal": "+44 2345678"
            }
        }"#;

    let mut reader = io::BufReader::new(data.as_bytes());

    let new_name = "George";
    let mut doc = Document::load(&mut reader, map_from_str).unwrap();
    doc.update("name", new_name.into(), None::<QryClause>).expect("update panic message");

    assert_eq!(doc.content().get("name").unwrap().as_str().unwrap(), new_name);
}

#[test]
fn json_document_delete() {
    let data = r#"
        {
            "name": "John",
            "age": 43,
            "address": {
                "zip": 7777,
                "city": "Edoras",
                "state": "Rohan"
            },
            "phones": {
                "office": "+44 1234567",
                "personal": "+44 2345678"
            }
        }"#;

    let mut reader = io::BufReader::new(data.as_bytes());

    let mut doc = Document::load(&mut reader, map_from_str).unwrap();
    doc.delete("address.zip", None::<QryClause>).unwrap();

    assert!(!contains_key("address.zip", doc.content()));
}

#[test]
fn json_document_select() {
    let data = r#"
        {
            "name": "John",
            "age": 43,
            "address": {
                "zip": 7777,
                "city": "Edoras",
                "state": "Rohan"
            },
            "phones": {
                "office": "+44 1234567",
                "personal": "+44 2345678"
            }
        }"#;

    let mut reader = io::BufReader::new(data.as_bytes());

    let doc = Document::load(&mut reader, map_from_str).unwrap();
    let result = doc.select(&["name", "age", "address.zip"], None::<QryClause>).unwrap();
    let expected : Vec<(String, Value)> = vec![("name".to_string(), "John".into()), ("age".to_string(), 43.into()), ("address.zip".to_string(), 7777.into())];

    assert_eq!(result, expected);
}

#[test]
fn json_document_select_with_query() {
    let data = r#"
        {
            "id": 1111,
            "name": "John",
            "age": 43,
            "address": {
                "zip": 7777,
                "city": "Edoras",
                "state": "Rohan"
            },
            "phones": {
                "office": "+44 1234567",
                "personal": "+44 2345678"
            }
        }"#;

    let mut reader = io::BufReader::new(data.as_bytes());

    let doc = Document::load(&mut reader, map_from_str).unwrap();
    let qry : QryClause = QueryClause::equal("id", 1111);
    let result = doc.select(&["name", "age", "address.zip"], Some(qry)).unwrap();
    let expected : Vec<(String, Value)> = vec![("name".to_string(), "John".into()), ("age".to_string(), 43.into()), ("address.zip".to_string(), 7777.into())];

    assert_eq!(result, expected);
}