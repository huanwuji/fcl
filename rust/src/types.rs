use std::any::Any;

#[inline]
pub fn cast<T: Any>(v: &dyn Any) -> &T {
    unsafe {
        &*(v as *const dyn Any as *const T)
    }
}

#[inline]
pub fn any<T: Any>(v: &T) -> &dyn Any {
    v as &dyn Any
}

#[test]
fn any_type() {
    let a = any(&32);
    let b = cast::<i32>(a);
    eprintln!("b = {:?}", b);
}

#[test]
fn any_test() {
//    let value = &"aa".to_string();
//    let value_any = value as &dyn Any;
//
//    // Try to convert our value to a `String`. If successful, we want to
//    // output the String`'s length as well as its value. If not, it's a
//    // different type: just print it out unadorned.
//    match value_any.downcast_ref::<String>() {
//        Some(as_string) => {
//            println!("String ({}): {}", as_string.len(), as_string);
//        }
//        None => {
//            println!("{:?}", value);
//        }
//    }
    let a = &32;
    let value = a as &dyn Any;
    eprintln!("value = {:?}", value);
    unsafe {
        let b = &*(value as *const dyn Any as *const i32);
        eprintln!("b = {:?}", b);
    }
}