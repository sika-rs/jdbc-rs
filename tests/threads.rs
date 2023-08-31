#[cfg(not(feature = "async"))]
#[test]
pub fn test() -> Result<(), jdbc::errors::Error> {
    use std::sync::Arc;

    let ds = Arc::new(util::sqlite());
    let ds1 = ds.clone();
    let h1 = std::thread::spawn(move || {
        let conn = ds.get_connection();
        match conn {
            Ok(_) => true,
            Err(_) => false,
        }
    });
    let h2 = std::thread::spawn(move || {
        let conn = ds1.get_connection();
        match conn {
            Ok(_) => true,
            Err(_) => false,
        }
    });

    assert_eq!(
        h1.join().expect("Couldn't join on the associated thread"),
        true
    );
    assert_eq!(
        h2.join().expect("Couldn't join on the associated thread"),
        true
    );

    Ok(())
}

#[macro_use]
extern crate lazy_static;
mod util;
