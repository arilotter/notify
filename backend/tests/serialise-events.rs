use notify_backend::prelude::*;
use serde_json::json;

#[test]
fn events_are_debuggable() {
    assert_eq!(format!("{:?}", EventKind::Any), String::from("Any"));

    assert_eq!(
        format!(
            "{:?}",
            EventKind::Access(AccessKind::Open(AccessMode::Execute))
        ),
        String::from("Access(Open(Execute))")
    );

    let mut attrs = AnyMap::new();
    attrs.insert(event::Info("unmount".into()));

    assert_eq!(
        format!(
            "{:?}",
            Event {
                kind: EventKind::Remove(RemoveKind::Other),
                path: Some("/example".into()),
                attrs
            }
        ),
        String::from(
            "Event { kind: Remove(Other), path: Some(\"/example\"), attr:tracker: None, attr:info: Some(\"unmount\"), attr:source: None }"
        )
    );
}


#[cfg(feature = "serde")]
#[test]
fn events_are_serializable() {
    assert_eq!(json!(EventKind::Any), json!(
        "any"
    ));

    assert_eq!(json!(EventKind::Other), json!(
        "other"
    ));

    assert_eq!(json!(Event {
        kind: EventKind::Access(AccessKind::Open(AccessMode::Execute)),
        path: None,
        attrs: AnyMap::new(),
    }), json!({
        "type": { "access": { "kind": "open", "mode": "execute" } },
        "path": null,
        "attrs": {},
    }));

    let mut attrs = AnyMap::new();
    attrs.insert(event::Info("unmount".into()));

    assert_eq!(json!(Event {
        kind: EventKind::Remove(RemoveKind::Other),
        path: Some("/example".into()),
        attrs
    }), json!({
        "type": { "remove": { "kind": "other" } },
        "path": "/example",
        "attrs": { "info": "unmount" }
    }));
}

#[cfg(feature = "serde")]
#[test]
fn access_events_are_serializable() {
    assert_eq!(json!(EventKind::Access(AccessKind::Any)), json!({
        "access": { "kind": "any" }
    }));

    assert_eq!(json!(EventKind::Access(AccessKind::Read)), json!({
        "access": { "kind": "read" }
    }));

    assert_eq!(json!(EventKind::Access(AccessKind::Open(AccessMode::Any))), json!({
        "access": { "kind": "open", "mode": "any" }
    }));

    assert_eq!(json!(EventKind::Access(AccessKind::Open(AccessMode::Execute))), json!({
        "access": { "kind": "open", "mode": "execute" }
    }));

    assert_eq!(json!(EventKind::Access(AccessKind::Open(AccessMode::Read))), json!({
        "access": { "kind": "open", "mode": "read" }
    }));

    assert_eq!(json!(EventKind::Access(AccessKind::Close(AccessMode::Write))), json!({
        "access": { "kind": "close", "mode": "write" }
    }));

    assert_eq!(json!(EventKind::Access(AccessKind::Close(AccessMode::Other))), json!({
        "access": { "kind": "close", "mode": "other" }
    }));

    assert_eq!(json!(EventKind::Access(AccessKind::Other)), json!({
        "access": { "kind": "other" }
    }));
}

#[cfg(feature = "serde")]
#[test]
fn create_events_are_serializable() {
    assert_eq!(json!(EventKind::Create(CreateKind::Any)), json!({
        "create": { "kind": "any" }
    }));

    assert_eq!(json!(EventKind::Create(CreateKind::File)), json!({
        "create": { "kind": "file" }
    }));

    assert_eq!(json!(EventKind::Create(CreateKind::Folder)), json!({
        "create": { "kind": "folder" }
    }));

    assert_eq!(json!(EventKind::Create(CreateKind::Other)), json!({
        "create": { "kind": "other" }
    }));
}

#[cfg(feature = "serde")]
#[test]
fn modify_events_are_serializable() {
    assert_eq!(json!(EventKind::Modify(ModifyKind::Any)), json!({
        "modify": { "kind": "any" }
    }));

    assert_eq!(json!(EventKind::Modify(ModifyKind::Data(DataChange::Any))), json!({
        "modify": { "kind": "data", "mode": "any" }
    }));

    assert_eq!(json!(EventKind::Modify(ModifyKind::Data(DataChange::Size))), json!({
        "modify": { "kind": "data", "mode": "size" }
    }));

    assert_eq!(json!(EventKind::Modify(ModifyKind::Data(DataChange::Content))), json!({
        "modify": { "kind": "data", "mode": "content" }
    }));

    assert_eq!(json!(EventKind::Modify(ModifyKind::Data(DataChange::Other))), json!({
        "modify": { "kind": "data", "mode": "other" }
    }));

    assert_eq!(json!(EventKind::Modify(ModifyKind::Metadata(MetadataKind::Any))), json!({
        "modify": { "kind": "metadata", "mode": "any" }
    }));

    assert_eq!(json!(EventKind::Modify(ModifyKind::Metadata(MetadataKind::AccessTime))), json!({
        "modify": { "kind": "metadata", "mode": "access-time" }
    }));

    assert_eq!(json!(EventKind::Modify(ModifyKind::Metadata(MetadataKind::WriteTime))), json!({
        "modify": { "kind": "metadata", "mode": "write-time" }
    }));

    assert_eq!(json!(EventKind::Modify(ModifyKind::Metadata(MetadataKind::Permissions))), json!({
        "modify": { "kind": "metadata", "mode": "permissions" }
    }));

    assert_eq!(json!(EventKind::Modify(ModifyKind::Metadata(MetadataKind::Ownership))), json!({
        "modify": { "kind": "metadata", "mode": "ownership" }
    }));

    assert_eq!(json!(EventKind::Modify(ModifyKind::Metadata(MetadataKind::Extended))), json!({
        "modify": { "kind": "metadata", "mode": "extended" }
    }));

    assert_eq!(json!(EventKind::Modify(ModifyKind::Metadata(MetadataKind::Other))), json!({
        "modify": { "kind": "metadata", "mode": "other" }
    }));

    assert_eq!(json!(EventKind::Modify(ModifyKind::Name(RenameMode::Any))), json!({
        "modify": { "kind": "rename", "mode": "any" }
    }));

    assert_eq!(json!(EventKind::Modify(ModifyKind::Name(RenameMode::To))), json!({
        "modify": { "kind": "rename", "mode": "to" }
    }));

    assert_eq!(json!(EventKind::Modify(ModifyKind::Name(RenameMode::From))), json!({
        "modify": { "kind": "rename", "mode": "from" }
    }));

    assert_eq!(json!(EventKind::Modify(ModifyKind::Name(RenameMode::Other))), json!({
        "modify": { "kind": "rename", "mode": "other" }
    }));

    assert_eq!(json!(EventKind::Modify(ModifyKind::Other)), json!({
        "modify": { "kind": "other" }
    }));
}

#[cfg(feature = "serde")]
#[test]
fn remove_events_are_serializable() {
    assert_eq!(json!(EventKind::Remove(RemoveKind::Any)), json!({
        "remove": { "kind": "any" }
    }));

    assert_eq!(json!(EventKind::Remove(RemoveKind::File)), json!({
        "remove": { "kind": "file" }
    }));

    assert_eq!(json!(EventKind::Remove(RemoveKind::Folder)), json!({
        "remove": { "kind": "folder" }
    }));

    assert_eq!(json!(EventKind::Remove(RemoveKind::Other)), json!({
        "remove": { "kind": "other" }
    }));
}
