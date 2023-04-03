fn main() {}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_box_with_recursive_type() {
        // https://doc.rust-lang.org/book/ch15-01-box.html

        // cannot compile:
        // error[E0072]: recursive type `Node` has infinite size
        // struct Node {
        //     child: Option<Node>,
        //     value: usize,
        // }
        // let node = Node { value: 0, child: Some(Node{value: 1, child: None}) };

        // using Box on the child Node make it compile because child has now a known size
        // (here size of Option->Box->reference to the heap)
        // which is a requirement to have code on the stack
        // the child Node is stored on the heap and can have an unknown size at compile time
        struct Node {
            child: Option<Box<Node>>,
            value: usize,
        }
        let node = Node {
            value: 0,
            child: Some(Box::new(Node {
                value: 1,
                child: None,
            })),
        };

        assert_eq!(node.child.expect("child").value, 1);
    }

    #[test]
    fn test_box_with_dynamic_type() {
        // https://doc.rust-lang.org/book/ch15-01-box.html
        trait Foo {
            fn value(&self) -> usize;
        }
        struct FooImpl {
            x: usize,
        }

        impl Foo for FooImpl {
            fn value(&self) -> usize {
                self.x
            }
        }

        // cannot compile:
        // error[E0277]: the size for values of type `(dyn Foo + 'static)` cannot be known at compilation time
        // struct Bar {
        //     foo: dyn Foo,
        // }
        // let bar = Bar { foo: FooImpl{x: 0} };

        // foo field is not a concrete type but any type that implements the trait Foo
        // thus its size can vary from each implementation
        // boxing it solves our problem of unknown compile time size for values on the stack
        struct Bar {
            foo: Box<dyn Foo>,
        }
        let bar = Bar {
            foo: Box::new(FooImpl { x: 123 }),
        };

        assert_eq!(bar.foo.value(), 123);
    }

    #[test]
    fn test_rc() {
        // https://doc.rust-lang.org/book/ch15-04-rc.html
        println!("[test_rc] start");

        struct Foo {
            _value: String,
        }

        // error[E0382]: use of moved value: `a`
        // let a = Foo{value: "a".to_string()};
        // let b = a;
        // let c = a; // value used here after move

        // to be able to have multiple read only owners on a variable,
        // we use Reference Counted (Rc) pointers
        let a = Rc::new(Foo {
            _value: "a".to_string(),
        });
        println!("count after creating a = {}", Rc::strong_count(&a)); // 1

        // adding a first reference
        let _b = Rc::clone(&a);
        println!("count after creating b = {}", Rc::strong_count(&a)); // 2
        {
            // adding a second reference inside the scope {}
            let _c = Rc::clone(&a);
            println!("count after creating c = {}", Rc::strong_count(&a)); // 3
                                                                           // second reference goes out of scope
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2
        println!("[test_rc] end");
    }

    #[test]
    fn test_ref_cell() {
        // https://doc.rust-lang.org/book/ch15-05-interior-mutability.html

        // compiler borrow checker, you can either:
        // - one mutable reference
        // - any number of immutable references

        println!("[test_ref_cell] start");

        // compiles but fails at runtime because RefCell are checked at runtime
        // we cannot have 2 borrowing references at the same time (on mut_borrow_2 creation)
        // thread 'tests::test_ref_cell' panicked at 'already borrowed: BorrowMutError'

        // let x: RefCell<Vec<usize>> = RefCell::new(vec![]);
        // let mut mut_borrow_1 = x.borrow_mut();
        // let mut mut_borrow_2 = x.borrow_mut();
        // mut_borrow_1.push(1);
        // mut_borrow_2.push(2);
        // println!("{:?}", x);

        // whereas the following works
        let x: RefCell<Vec<usize>> = RefCell::new(vec![]);
        {
            let mut mut_borrow_1 = x.borrow_mut();
            mut_borrow_1.push(1);
            // dropping mut_borrow_1 => we can mutably borrow x again
        }
        let mut mut_borrow_2 = x.borrow_mut();
        mut_borrow_2.push(2);
        println!("{:?}", x);

        println!("[test_ref_cell] end");
    }

    #[test]
    fn test_arc() {
        // https://doc.rust-lang.org/std/sync/struct.Arc.html
        println!("[test_arc] start");

        #[derive(Debug)]
        struct Foo {
            _value: usize,
        }

        // cannot compile
        // error[E0382]: use of moved value: `x`
        // x cannot be shared across threads
        // let x = Foo{_value: 0};
        // for _ in 0..3 {
        //     thread::spawn(move || {
        //         println!("{x:?}");
        //     });
        // }

        let x = Arc::new(Foo { _value: 5 });

        for i in 0..3 {
            let x = Arc::clone(&x);

            thread::spawn(move || {
                println!("[test_arc] thread {i}, {x:?}, ended");
            });
        }
    }

    #[test]
    fn test_cow() {
        // https://doc.rust-lang.org/std/borrow/enum.Cow.html
        println!("[test_cow] start");

        fn abs_all(input: &mut Cow<[i32]>) {
            for i in 0..input.len() {
                let v = input[i];
                if v < 0 {
                    // Clones into a vector if not already owned.
                    input.to_mut()[i] = -v;
                }
            }
        }

        println!("No clone occurs because `input` doesn't need to be mutated.");
        let input = [0, 1, 2];
        let mut input_as_cow = Cow::from(&input[..]);
        abs_all(&mut input_as_cow);
        println!("input address: {:p}", input.as_ptr());
        println!("input_as_cow address: {:p}\n", input_as_cow.as_ptr());
        assert_eq!(input.as_ptr(), input_as_cow.as_ptr()); // clone did not occur

        println!("Clone occurs because `input` needs to be mutated.");
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        abs_all(&mut input);
        println!("input address: {:p}", input.as_ptr());
        println!("input_as_cow address: {:p}\n", input_as_cow.as_ptr());
        assert_ne!(input.as_ptr(), input_as_cow.as_ptr()); // clone occurred

        println!("No clone occurs because `input` is already owned");
        let mut input = Cow::from(vec![-1, 0, 1]);
        let initial_address = format!("{:p}", input.as_ptr());
        println!("initial address: {}", initial_address);
        println!("{input:?}");
        abs_all(&mut input);
        let address_afterward = format!("{:p}", input.as_ptr());
        println!("address after modification: {}", address_afterward);
        println!("{input:?}");
        assert_eq!(initial_address, address_afterward); // clone did not occur
        println!("[test_cow] end");
    }
}
