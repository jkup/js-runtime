use rusty_v8 as v8;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the JavaScript file from disk
    let filename = "path/to/your/javascript/file.js";
    let source_code = fs::read_to_string(filename)?;

    // Initialize the V8 engine
    let platform = v8::new_default_platform()?;
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    // Create a new V8 isolate and scope
    let mut isolate = v8::Isolate::new(Default::default());
    let scope = &mut v8::HandleScope::new(&mut isolate);

    // Create a new context and enter it
    let context = v8::Context::new(scope);
    let mut context_scope = v8::ContextScope::new(scope, context);

    // Compile and run the JavaScript code
    let source = v8::String::new(scope, &source_code).unwrap();
    let script = v8::Script::compile(scope, context, source, None)?;
    let result = script.run(scope, context)?;

    // Convert the result to a Rust string and print it
    let result_string = result.to_string(scope).unwrap();
    println!("{}", result_string);

    // Clean up the V8 engine
    drop(context_scope);
    drop(isolate);
    v8::V8::dispose();
    v8::V8::shutdown_platform();

    Ok(())
}
