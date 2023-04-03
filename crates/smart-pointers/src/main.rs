fn main() {}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    #[test]
    fn test_box_with_recursive_type() {
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
        // more at https://doc.rust-lang.org/book/ch15-01-box.html
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
        struct Foo {
            _value: String,
        }

        // error[E0382]: use of moved value: `a`
        // let a = Foo{value: "a".to_string()};
        // let b = a;
        // let c = a; // value used here after move

        // to be able to have multiple read only owners on a variable,
        // we use Reference Counted (Rc) pointers

        // note: example very similar to https://doc.rust-lang.org/book/ch15-04-rc.html
        // because rust's book is great

        println!("[test_rc] start");
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
    fn test_ref_cell() {}

    #[test]
    fn test_arc() {}

    #[test]
    fn test_cow() {}
}
