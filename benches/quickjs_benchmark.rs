use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json::json;
use serde_json::Value as JsonValue;
use quick_js::{Context, JsValue};


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
    
        c.bench_function("quickjs_benchmark", |b| b.iter(|| context.call_function("apply_rules", vec![event,rules]).unwrap()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);