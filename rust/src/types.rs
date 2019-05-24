use std::any::Any;

impl dyn Any {
    pub fn downcast_ref_unchecked<T: Any>(&self) -> Option<&T> {
        unsafe {
            Some(&*(self as *const dyn Any as *const T))
        }
    }
}

impl<T> From<T> for Any {
    fn from(v: T) -> &Self {
        v as &dyn Any
    }
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

//    match value.downcast_ref::<String>() {}
}