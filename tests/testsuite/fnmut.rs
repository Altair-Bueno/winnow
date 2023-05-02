#![cfg(feature = "alloc")]

use winnow::combinator::repeat;
use winnow::Parser;

#[test]
#[cfg(feature = "std")]
fn parse() {
    let mut counter = 0;

    let res = {
        let mut parser = repeat::<_, _, Vec<_>, (), _>(0.., |i| {
            counter += 1;
            "abc".parse_next(i)
        });

        parser.parse_next("abcabcabcabc").unwrap()
    };

    println!("res: {:?}", res);
    assert_eq!(counter, 5);
}

#[test]
fn accumulate() {
    let mut v = Vec::new();

    let (_, count) = {
        let mut parser = repeat::<_, _, usize, (), _>(0.., |i| {
            let (i, o) = "abc".parse_next(i)?;
            v.push(o);
            Ok((i, ()))
        });
        parser.parse_next("abcabcabcabc").unwrap()
    };

    println!("v: {:?}", v);
    assert_eq!(count, 4);
    assert_eq!(v.len(), 4);
}
