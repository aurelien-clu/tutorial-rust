fn main() {}

#[cfg(test)]
mod tests {

    #[test]
    fn test_box_with_recursive_type() {
        // cannot compile:
        // error[E0072]: recursive type `Node` has infinite size
        // struct Node {
        //     child: Option<Node>,
        //     value: usize,
        // }
        // let node = Node { value: 0, child: Some(Node{value: 1, child: None}) };

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

        struct Bar {
            foo: Box<dyn Foo>,
        }
        let bar = Bar {
            foo: Box::new(FooImpl { x: 123 }),
        };

        assert_eq!(bar.foo.value(), 123);
    }
}
