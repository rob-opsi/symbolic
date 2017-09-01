extern crate symbolic_demangle;

use symbolic_demangle::{demangle, DemangleOptions};


fn assert_mangle(input: &str, output: Option<&str>, opts: DemangleOptions) {
    if let Some(rv) = demangle(input, &opts).unwrap() {
        assert_eq!(Some(rv.as_str()), output);
    } else {
        assert_eq!(None, output);
    }
}


#[test]
fn test_rust_demangle() {
    assert_mangle("_ZN3foo3barE", Some("foo::bar"), Default::default());
}

#[test]
fn test_swift_demangle() {
    assert_mangle("_TTWVSC29UIApplicationLaunchOptionsKeys21_ObjectiveCBridgeable5UIKitZFS0_36_unconditionallyBridgeFromObjectiveCfGSqwx15_ObjectiveCType_x", Some("protocol witness for static _ObjectiveCBridgeable._unconditionallyBridgeFromObjectiveC(_:) in conformance UIApplicationLaunchOptionsKey"), Default::default());
}

#[test]
fn test_cpp_demangle() {
    assert_mangle("_Z28JS_GetPropertyDescriptorByIdP9JSContextN2JS6HandleIP8JSObjectEENS2_I4jsidEENS1_13MutableHandleINS1_18PropertyDescriptorEEE",
                  Some("JS_GetPropertyDescriptorById"), Default::default());
    assert_mangle("_Z28JS_GetPropertyDescriptorByIdP9JSContextN2JS6HandleIP8JSObjectEENS2_I4jsidEENS1_13MutableHandleINS1_18PropertyDescriptorEEE",
                  Some("JS_GetPropertyDescriptorById(JSContext*, JS::Handle<JSObject*>, JS::Handle<jsid>, JS::MutableHandle<JS::PropertyDescriptor>)"), DemangleOptions {
        with_arguments: true,
        ..Default::default()
    });
}

#[test]
fn test_no_match() {
    assert_mangle("foo", None, Default::default());
    assert_mangle("_ZN3foo3barE", None, DemangleOptions {
        languages: vec![],
        ..Default::default()
    });
}
