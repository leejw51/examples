use neon::prelude::*;
use neon::register_module;

/// Rust panics will throw an Error in Node
fn throw_error(mut cx: FunctionContext) -> JsResult<JsString> {
    // Get the value of the first argument and assert that it is a string
    let arg0 = cx.argument::<JsString>(0)?.value();

    // If the argument does not contain hello, throw an Error
    if !arg0.contains("hello") {
        // Equaivalent to `throw new Error('Expected you to say hello')` in JS
        panic!("Expected you to say hello");
    }

    Ok(cx.string("hello to you too!"))
}

/// Throw TypeError
fn throw_type_error(mut cx: FunctionContext) -> JsResult<JsValue> {
    let _foo: JsResult<JsError> = cx.throw_type_error("not enough arguments")?;
    Ok(cx.string("throw_error node").upcast())
}

/// Throw custom errors
fn throw_custom_error(mut cx: FunctionContext) -> JsResult<JsError> {
    let arg0 = cx.argument::<JsString>(0)?.value();

    match arg0.as_str() {
        "type_error" => cx.throw_type_error("throwing a TypeError"),
        "range_error" => cx.throw_range_error("throwing a RangeError"),
        "error" => cx.throw_error("throwing an Error"),
        _ => panic!("please pass an expected error type"),
    }
}

/// Creating error objects. This function will not throw errors. It will
/// only create them and return them
fn create_error_obj(mut cx: FunctionContext) -> JsResult<JsError> {
    let arg0 = cx.argument::<JsString>(0)?.value();

    match arg0.as_str() {
        "type_error" => cx.type_error("creating a TypeError"),
        "range_error" => cx.range_error("creating a RangeError"),
        "error" => cx.error("creating an Error"),
        _ => panic!("please pass an expected error type"),
    }
}

register_module!(mut m, {
    m.export_function("throwError", throw_error)?;
    m.export_function("throwCustomError", throw_custom_error)?;
    m.export_function("throwTypeError", throw_type_error)?;
    m.export_function("createErrorObj", create_error_obj)?;
    Ok(())
});
