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
    view.enter(|x| x.a = 1);
    view.enter(|x| {
        let mut y = Box::new(x.a);
        *y += 5;
    });
    let mut view1 = view.clone();
    // view1.enter(|x| println!("x.a={}", x.a));
    view1.enter(|x| x.y = Box::new(123));
    // view1.enter(|x| println!("x.y={}", x.y));
    // view.enter(|x| println!("x.a={}", x.a));
    view.enter(|x| x.y = Box::new(124));
    // view.enter(|x| println!("x.y={}", x.y));
    let y = view.enter(|x| x.a);
    println!("{y}");
}
