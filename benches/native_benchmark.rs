use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json::json;
use serde_json::Value as JsonValue;
use js_sandbox::{Script};
use std::fs;

// Pure rust
fn apply_rules_rust(payload:& JsonValue) -> JsonValue {
    for (name, obj) in payload["event"].as_object().unwrap().iter() {
        
    }
    payload.get("event");
    return payload.clone();
}

fn criterion_benchmark(c: &mut Criterion) {
    // Faking an event payload 
        // We will have that in rust application   
        let event = r#"
        {
            "event" : "identity",
            "partnerUserId" : 123,
            "gauthUserId" : 122,
            "firstName" : "john",
            "lastName" : "doe",
            "sonicId" : 124,
            "realm" : 123,
            "profileId" : 434,
            "session":{
                "playbackstart":1
            }
        }
        "#; 
        // Faking sample rules
        let rules = r#"
                [
                {
                    "ruleName": "Add Email",
                    "logic" : "function(event){return event['email'] = event['name'] + event['lastName'] + 'gmail.com'}",
                    "priority" : 2
                },
                {
                    "ruleName": "Make Full Name",
                    "logic" : "function(event){return event['fullName'] = event['name'] + event['lastName']}",
                    "priority" : 1
                    }
                ]  
        "#;
        let event_obj: JsonValue = serde_json::from_str(event).unwrap();
        let rules_obj: JsonValue = serde_json::from_str(rules).unwrap();
        // // js_sandbox allows only one arg
        let payload = json!({
            "event": event_obj,
            "rules": rules_obj
        });

        c.bench_function("native benchmark", |b| b.iter(|| apply_rules_rust(& payload)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);