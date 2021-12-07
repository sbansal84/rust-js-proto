#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};
    use std::fs;
    use serde_json::json;
    use serde_json::Value as JsonValue;
    use js_sandbox::{Script};
    use quick_js::{Context, JsValue};


    // // using js_sandbox (which is v8 underneath)
    fn apply_rules_js_sandbox(payload:& JsonValue,script:&mut Script) -> JsonValue {
        return script.call("apply_rules", &payload).unwrap();
    }
    // Pure rust
    fn apply_rules_rust(payload:& JsonValue) -> JsonValue {
        for (name, obj) in payload["event"].as_object().unwrap().iter() {
            // println!("{}", name);
        }
        return payload.clone();
    }

    #[bench]
    fn bench_pow(b: &mut Bencher) {

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


        // // begin js-sandbox
        // // load js module as string
        let js_code = fs::read_to_string("/Users/saurabhbansal/Desktop/rust-js-proto/js/index.js")
        .expect("Something went wrong reading the file");
        let mut script = Script::from_string(&js_code).expect("Initialization succeeds");
        println!("i am called");
        let event_obj: JsonValue = serde_json::from_str(event).unwrap();
        let rules_obj: JsonValue = serde_json::from_str(rules).unwrap();
        // // js_sandbox allows only one arg
        let payload = json!({
            "event": event_obj,
            "rules": rules_obj
        });
        // // end js-sanbox

        // quick-js-rs
        let context = Context::new().unwrap();
        context.eval(
            r#"
            function apply_rules(event,rules){
                var event = JSON.parse(event);
                var rules = JSON.parse(rules);       
                rules.sort(function(a,b){
                    return a.priority - b.priority
                }).map(function(rule){
                    var fn = new Function('return ' + rule['logic'])(); 
                    fn(event);
                })
                return event;
            };
    
        "#,
        ).unwrap();
        context.call_function("apply_rules", vec![event,rules]).unwrap();
        // end


            b.iter(|| {
                // Inner closure, the actual test
                for i in 1..10 {
                    // println!("Hello is {}","world");
                    // black_box(x.powf(y).powf(x));
                    // apply_rules_rust(& payload);
                    // apply_rules_js_sandbox(& payload,&mut script);
                    context.call_function("apply_rules", vec![event,rules]).unwrap();
                }
            });
    }
}



