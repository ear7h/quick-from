#![allow(dead_code)]

#[macro_use]
extern crate quick_from;

enum Enum1 {
    A,
    B,
    C,
}

#[derive(QuickFrom)]
enum Enum2 {
    X,
    Y,
    Z,
    #[quick_from]
    E1(Enum1),
}

#[test]
fn local() {
    let _x : Enum2 = From::from(Enum1::A);
}


#[derive(QuickFrom)]
enum Error {
    InvalidInput,
    #[quick_from]
    Io(std::io::Error),
}

#[test]
fn import() {
    fn my_read(s : &str) -> Result<Vec<u8>, Error> {
        if s.len() == 0 {
            return Err(Error::InvalidInput)
        }

        Ok(std::fs::read("hello.txt")?)
    }

    let _x = my_read("");
    let _x = my_read("hello.txt");

}

#[derive(QuickFrom)]
enum Enum3<'a, 'b, T> {
    #[quick_from]
    A(&'a [u32]),

    #[quick_from]
    B(&'b bool),

    C(T)
}

#[test]
fn lifetime() {
    let s : &[u32] = &[1, 2, 3];
    let _ : Enum3<i32> = s.into();
}

