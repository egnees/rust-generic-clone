use crate::{store::Store, view::View};

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
struct X {
    a: i32,
    y: Box<i32>,
}

#[test]
fn works() {
    let store = Store::new(16000, 20).unwrap();
    let mut view: View<X> = store.allocate().unwrap();
    println!("entering...");
    view.enter(|x| x.a = 1);
    view.enter(|x| println!("x.a={}", x.a));
    let mut view1 = view.clone();
    view1.enter(|x| println!("x.a={}", x.a));
    view1.enter(|x| x.y = Box::new(123));
    view1.enter(|x| println!("x.y={}", x.y));
    view.enter(|x| println!("x.a={}", x.a));
    view.enter(|x| x.y = Box::new(124));
    view.enter(|x| println!("x.y={}", x.y));
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
struct A<'a> {
    rt: smol::LocalExecutor<'a>,
}

#[test]
fn works1() {
    let store = Store::new(16000, 20).unwrap();
}
