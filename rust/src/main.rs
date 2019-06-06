//struct Haha<'a> {
//    arr: &'a [i32]
//}
//
//struct Gaga {}
//
//impl<'a> Gaga {
//    fn re(&self) -> Haha<'a> {
//        let vec = vec![1, 2, 3];
//        Haha { arr: vec.as_slice() }
//    }
//}
//
//fn main() {
//    let gaga = Gaga {};
//    gaga.re();
//}
