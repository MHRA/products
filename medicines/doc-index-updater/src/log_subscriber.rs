use chrono::offset::Utc;
use chrono::DateTime;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::sleep;
use std::time::SystemTime;

use tracing::debug;
use tracing_core::{
    event::Event,
    metadata::Metadata,
    span::{Attributes, Id, Record},
    subscriber::Subscriber,
};

use tracing::Level;

use tracing_serde::AsSerde;

use serde_json::json;

pub struct JsonSubscriber {
    next_id: AtomicUsize, // you need to assign span IDs, so you need a counter
}

impl JsonSubscriber {
    pub fn new() -> JsonSubscriber {
        JsonSubscriber {
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Subscriber for JsonSubscriber {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        let json = json!({
        "enabled": {
            "metadata": metadata.as_serde(),
        }});
        //println!("{}", json);
        true
    }

    fn new_span(&self, attrs: &Attributes<'_>) -> Id {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let id = Id::from_u64(id as u64);
        let json = json!({
        "new_span": {
            "attributes": attrs.as_serde(),
            "id": id.as_serde(),
        }});
        //println!("{}", json);
        id
    }

    fn record(&self, span: &Id, values: &Record<'_>) {
        let json = json!({
        "record": {
            "span": span.as_serde(),
            "values": values.as_serde(),
        }});
        println!("{}", json);
    }

    fn record_follows_from(&self, span: &Id, follows: &Id) {
        let json = json!({
        "record_follows_from": {
            "span": span.as_serde(),
            "follows": follows.as_serde(),
        }});
        println!("{}", json);
    }

    fn event(&self, event: &Event<'_>) {
        if event.metadata().level() != &Level::INFO {
            return;
        }

        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();

        let json = json!({
            "timestamp": datetime.to_rfc3339(),
            "level": event.metadata().level().to_string() ,
            "service_name": event.metadata().module_path(),
            "correlation_id": "92bb6eb5-b8c8-4ad3-8e5a-65bb71e34729", // TODO
            // "event": {
            //   "label": "DOCUMENT_MANAGER_CREATE",
            //   "payload": {


            //   }
            // },

            "event_raw": event.as_serde()
        });

        println!("{}", json);
    }

    fn enter(&self, span: &Id) {
        let json = json!({
            "enter": span.as_serde(),
        });
        //println!("{}", json);
    }

    fn exit(&self, span: &Id) {
        let json = json!({
            "exit": span.as_serde(),
        });
        //println!("{}", json);
    }
}
