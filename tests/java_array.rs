extern crate rucaja;

use rucaja::{Jvm, JvmAttachment, JvmClass, JvmMethod, JvmObjectArray, JvmObject, jvalue_from_jobject, jvalue_from_jint};

#[test]
fn test_java_arrays() {
    unsafe {
        let jvm = Jvm::new(&[
            "-Xcheck:jni"
        ]);

        // Attach the current native thread to the JVM.
        let jvm_attachment = JvmAttachment::new(jvm.jvm());

        let integer_clazz = JvmClass::get_class(&jvm_attachment, "java/lang/Integer").unwrap();

        let integer_constructor = JvmMethod::get_constructor(
            &jvm_attachment,
            &integer_clazz,
            "(I)V"
        ).unwrap();

        let integer_jvm_ptr = JvmMethod::call_constructor(
            &jvm_attachment,
            &integer_clazz,
            &integer_constructor,
            vec![
                jvalue_from_jint(42)
            ].as_ptr()
        );

        let integer_object = JvmObject::from_jvm_ptr(&jvm_attachment, integer_jvm_ptr).unwrap();

        let arrays_clazz = JvmClass::get_class(&jvm_attachment, "java/util/Arrays").unwrap();

        let binary_search_method = JvmMethod::get_static_method(
            &jvm_attachment,
            &arrays_clazz,
            "binarySearch",
            "([Ljava/lang/Object;Ljava/lang/Object;)I"
        ).unwrap();

        let array_length = 10;

        let integer_array_object = JvmObjectArray::new(
            &jvm_attachment,
            array_length,
            &integer_clazz,
            &integer_object
        ).unwrap();

        let result = JvmMethod::call_static_int_method(
            &jvm_attachment,
            &arrays_clazz,
            &binary_search_method,
            vec![
                jvalue_from_jobject(integer_array_object.jvm_ptr()),
                jvalue_from_jobject(integer_object.jvm_ptr())
            ].as_ptr()
        );

        assert!( 0 <= result && result < array_length);
    }
}
