extern crate colored;
use self::colored::*;
use std::ops::Deref;

// # Smart pointers / Deref
//
// Simple user implementation of Box to understant Deref trait
// Big difference with original Box<T>: we are not storing data on the heap.
// More infos on https://doc.rust-lang.org/book/2018-edition/ch15-02-deref.html
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// The MyBox type `T` is a tuple of one element (accessed bellow via &self.0)
impl<T> Deref for MyBox<T> {
    type Target = T; // syntax defining an associated type for the Deref trait to use

    // - Returns a reference to the value we want to access with the * operator
    // - Not directly a value (because of the ownership system)
    // - If the deref method returned the value directly instead of a reference to the value,
    // the value would be moved out of self.
    // - We don’t want to take ownership of the inner value inside MyBox<T>
    // in this case or in most cases where we use the dereference operator
    fn deref(&self) -> &T {
        &self.0
    }
}

// # Smart pointers / Drop
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    // `drop` is implicitly called when the instance goes out of scope
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

pub fn run() -> i32 {
    println!("{}", "smartpointer".bold());
    deref();
    deref_coercion();
    println!("{}", "---- implicit drop -------".bold());
    implicit_drop();
    println!("{}", "---- explicit drop -------".bold());
    explicit_drop();
    println!("--------------------------");
    return 0;
}

fn deref() {
    let five_box = Box::new(5);
    let five_my_box = MyBox::new(5);
    println!(
        "{} {} {}",
        *five_box,
        *five_my_box,
        *five_box == *five_my_box
    );
}

fn deref_coercion() {
    // # Deref coercion
    //
    // Deref coercion converts:
    // - a reference to a type that implements Deref
    // -> into a reference to a type that Deref can convert the original type into
    let m = MyBox::new(String::from("Rust"));
    // Explanation of the following:
    // - Calling `hello` with `&m` which is a reference to `MyBox<String>`
    // - Because we implemented `Deref` on `MyBox<T>`
    //   - Rust can turn `&MyBox<String>` into `&String` by calling `deref`
    // - The standard library provides an implementation of `Deref` on `String` that returns a string slice
    //   - Rust calls `deref` again to turn the `&String` into `&str`
    // - `&str` type matches the type `hello` function accepts
    hello(&m);
    // Without Deref coercion, the syntax would be:
    // - The (*m) dereferences the MyBox<String> into a String
    // - the & and [..] take a string slice of the String
    hello(&(*m)[..]);
}

fn implicit_drop() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // when going out of scope,
    // values will be dropped in reverse order of creation (last created, first dropped)
}

fn explicit_drop() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c); // explicit drop using `std::mem::drop`
    println!("CustomSmartPointer dropped before the end of scope.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_deref_scalar() {
        let x = 5;
        let y = &x;

        assert_eq!(x, 5);
        assert_eq!(x, *y);
    }
    #[test]
    fn it_works_deref_box() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y); // deref operator works the same on Box<T> thanks to Deref trait
    }
    #[test]
    fn it_works_deref_my_box() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y); // deref operator works the same on MyBox<T> thanks to Deref trait
    }
}
