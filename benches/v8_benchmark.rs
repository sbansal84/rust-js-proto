use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json::json;
use serde_json::Value as JsonValue;
use js_sandbox::{Script};
use std::fs;


// // using js_sandbox (which is v8 underneath)
fn apply_rules_js_sandbox(payload:& JsonValue,script:&mut Script) -> JsonValue {
    return script.call("apply_rules", &payload).unwrap();
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

        // // begin js-sandbox
        // // load js module as string
        let js_code = fs::read_to_string("/Users/saurabhbansal/Desktop/rust-js-proto/js/index.js")
        .expect("Something went wrong reading the file");
        let mut script = Script::from_string(&js_code).expect("Initialization succeeds");
    
        c.bench_function("v8_benchmark", |b| b.iter(|| apply_rules_js_sandbox(& payload,&mut script)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);