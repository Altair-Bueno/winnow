#[cfg(test)]
mod test {
  use winnow::input::Streaming;
  use winnow::prelude::*;
  #[cfg(feature = "alloc")]
  use winnow::{branch::alt, bytes::tag_no_case, multi::many1};
  use winnow::{
    bytes::{tag, take, take_till, take_till1, take_until, take_while1},
    error::{self, Error, ErrorKind},
    ErrMode, IResult,
  };

  #[test]
  fn tag_succeed_str() {
    const INPUT: &str = "Hello World!";
    const TAG: &str = "Hello";
    fn test(input: &str) -> IResult<&str, &str> {
      tag(TAG)(input)
    }

    match test(INPUT) {
      Ok((extra, output)) => {
        assert!(extra == " World!", "Parser `tag` consumed leftover input.");
        assert!(
          output == TAG,
          "Parser `tag` doesn't return the tag it matched on success. \
           Expected `{}`, got `{}`.",
          TAG,
          output
        );
      }
      other => panic!(
        "Parser `tag` didn't succeed when it should have. \
         Got `{:?}`.",
        other
      ),
    };
  }

  #[test]
  fn tag_incomplete_str() {
    use winnow::bytes::tag;

    const INPUT: &str = "Hello";
    const TAG: &str = "Hello World!";

    let res: IResult<_, _, error::Error<_>> = tag(TAG)(Streaming(INPUT));
    match res {
      Err(ErrMode::Incomplete(_)) => (),
      other => {
        panic!(
          "Parser `tag` didn't require more input when it should have. \
           Got `{:?}`.",
          other
        );
      }
    };
  }

  #[test]
  fn tag_error_str() {
    const INPUT: &str = "Hello World!";
    const TAG: &str = "Random"; // TAG must be closer than INPUT.

    let res: IResult<_, _, error::Error<_>> = tag(TAG)(INPUT);
    match res {
      Err(ErrMode::Error(_)) => (),
      other => {
        panic!(
          "Parser `tag` didn't fail when it should have. Got `{:?}`.`",
          other
        );
      }
    };
  }

  #[cfg(feature = "alloc")]
  #[test]
  fn tag_case_insensitive_str() {
    fn test(i: &str) -> IResult<&str, &str> {
      tag_no_case("ABcd")(i)
    }
    assert_eq!(test("aBCdefgh"), Ok(("efgh", "aBCd")));
    assert_eq!(test("abcdefgh"), Ok(("efgh", "abcd")));
    assert_eq!(test("ABCDefgh"), Ok(("efgh", "ABCD")));
  }

  #[test]
  fn take_succeed_str() {
    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const CONSUMED: &str = "βèƒôřèÂßÇ";
    const LEFTOVER: &str = "áƒƭèř";

    let res: IResult<_, _, error::Error<_>> = take(9_usize)(INPUT);
    match res {
      Ok((extra, output)) => {
        assert!(
          extra == LEFTOVER,
          "Parser `take_s` consumed leftover input. Leftover `{}`.",
          extra
        );
        assert!(
          output == CONSUMED,
          "Parser `take_s` doesn't return the string it consumed on success. Expected `{}`, got `{}`.",
          CONSUMED,
          output
        );
      }
      other => panic!(
        "Parser `take_s` didn't succeed when it should have. \
         Got `{:?}`.",
        other
      ),
    };
  }

  #[test]
  fn take_incomplete_str() {
    use winnow::bytes::take;

    const INPUT: &str = "βèƒôřèÂßÇá";

    let res: IResult<_, _, Error<_>> = take(13_usize)(Streaming(INPUT));
    match res {
      Err(ErrMode::Incomplete(_)) => (),
      other => panic!(
        "Parser `take` didn't require more input when it should have. \
         Got `{:?}`.",
        other
      ),
    }
  }

  #[test]
  fn take_until_succeed_str() {
    const INPUT: &str = "βèƒôřèÂßÇ∂áƒƭèř";
    const FIND: &str = "ÂßÇ∂";
    const CONSUMED: &str = "βèƒôřè";
    const LEFTOVER: &str = "ÂßÇ∂áƒƭèř";

    let res: IResult<_, _, Error<_>> = take_until(FIND)(INPUT);
    match res {
      Ok((extra, output)) => {
        assert!(
          extra == LEFTOVER,
          "Parser `take_until`\
           consumed leftover input. Leftover `{}`.",
          extra
        );
        assert!(
          output == CONSUMED,
          "Parser `take_until`\
           doesn't return the string it consumed on success. Expected `{}`, got `{}`.",
          CONSUMED,
          output
        );
      }
      other => panic!(
        "Parser `take_until` didn't succeed when it should have. \
         Got `{:?}`.",
        other
      ),
    };
  }

  #[test]
  fn take_until_incomplete_str() {
    use winnow::bytes::take_until;

    const INPUT: &str = "βèƒôřè";
    const FIND: &str = "βèƒôřèÂßÇ";

    let res: IResult<_, _, Error<_>> = take_until(FIND)(Streaming(INPUT));
    match res {
      Err(ErrMode::Incomplete(_)) => (),
      other => panic!(
        "Parser `take_until` didn't require more input when it should have. \
         Got `{:?}`.",
        other
      ),
    };
  }

  #[test]
  fn take_until_error_str() {
    use winnow::bytes::take_until;

    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const FIND: &str = "Ráñδô₥";

    let res: IResult<_, _, Error<_>> = take_until(FIND)(Streaming(INPUT));
    match res {
      Err(ErrMode::Incomplete(_)) => (),
      other => panic!(
        "Parser `take_until` didn't fail when it should have. \
         Got `{:?}`.",
        other
      ),
    };
  }

  fn is_alphabetic(c: char) -> bool {
    (c as u8 >= 0x41 && c as u8 <= 0x5A) || (c as u8 >= 0x61 && c as u8 <= 0x7A)
  }

  #[test]
  fn take_while_str() {
    use winnow::Needed;

    use winnow::bytes::take_while;

    fn f(i: Streaming<&str>) -> IResult<Streaming<&str>, &str> {
      take_while(is_alphabetic)(i)
    }
    let a = "";
    let b = "abcd";
    let c = "abcd123";
    let d = "123";

    assert_eq!(f(Streaming(a)), Err(ErrMode::Incomplete(Needed::new(1))));
    assert_eq!(f(Streaming(b)), Err(ErrMode::Incomplete(Needed::new(1))));
    assert_eq!(f(Streaming(c)), Ok((Streaming(d), b)));
    assert_eq!(f(Streaming(d)), Ok((Streaming(d), a)));
  }

  #[test]
  fn take_while_succeed_none_str() {
    use winnow::bytes::take_while;

    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const CONSUMED: &str = "";
    const LEFTOVER: &str = "βèƒôřèÂßÇáƒƭèř";
    fn while_s(c: char) -> bool {
      c == '9'
    }
    fn test(input: &str) -> IResult<&str, &str> {
      take_while(while_s)(input)
    }
    match test(INPUT) {
      Ok((extra, output)) => {
        assert!(
          extra == LEFTOVER,
          "Parser `take_while` consumed leftover input."
        );
        assert!(
          output == CONSUMED,
          "Parser `take_while` doesn't return the string it consumed on success. \
           Expected `{}`, got `{}`.",
          CONSUMED,
          output
        );
      }
      other => panic!(
        "Parser `take_while` didn't succeed when it should have. \
         Got `{:?}`.",
        other
      ),
    };
  }

  #[test]
  fn take_while_succeed_some_str() {
    use winnow::bytes::take_while;

    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const CONSUMED: &str = "βèƒôřèÂßÇ";
    const LEFTOVER: &str = "áƒƭèř";
    fn while_s(c: char) -> bool {
      matches!(c, 'β' | 'è' | 'ƒ' | 'ô' | 'ř' | 'Â' | 'ß' | 'Ç')
    }
    fn test(input: &str) -> IResult<&str, &str> {
      take_while(while_s)(input)
    }
    match test(INPUT) {
      Ok((extra, output)) => {
        assert!(
          extra == LEFTOVER,
          "Parser `take_while` consumed leftover input."
        );
        assert!(
          output == CONSUMED,
          "Parser `take_while` doesn't return the string it consumed on success. \
           Expected `{}`, got `{}`.",
          CONSUMED,
          output
        );
      }
      other => panic!(
        "Parser `take_while` didn't succeed when it should have. \
         Got `{:?}`.",
        other
      ),
    };
  }

  #[test]
  fn test_take_while1_str() {
    use winnow::Needed;

    fn f(i: Streaming<&str>) -> IResult<Streaming<&str>, &str> {
      take_while1(is_alphabetic)(i)
    }
    let a = "";
    let b = "abcd";
    let c = "abcd123";
    let d = "123";

    assert_eq!(f(Streaming(a)), Err(ErrMode::Incomplete(Needed::new(1))));
    assert_eq!(f(Streaming(b)), Err(ErrMode::Incomplete(Needed::new(1))));
    assert_eq!(f(Streaming(c)), Ok((Streaming("123"), b)));
    assert_eq!(
      f(Streaming(d)),
      Err(ErrMode::Error(winnow::error::Error {
        input: Streaming(d),
        kind: ErrorKind::TakeWhile1
      }))
    );
  }

  #[test]
  fn take_while1_fn_succeed_str() {
    use winnow::bytes::take_while1;

    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const CONSUMED: &str = "βèƒôřèÂßÇ";
    const LEFTOVER: &str = "áƒƭèř";
    fn while1_s(c: char) -> bool {
      matches!(c, 'β' | 'è' | 'ƒ' | 'ô' | 'ř' | 'Â' | 'ß' | 'Ç')
    }
    fn test(input: &str) -> IResult<&str, &str> {
      take_while1(while1_s)(input)
    }
    match test(INPUT) {
      Ok((extra, output)) => {
        assert!(
          extra == LEFTOVER,
          "Parser `take_while1` consumed leftover input."
        );
        assert!(
          output == CONSUMED,
          "Parser `take_while1` doesn't return the string it consumed on success. \
           Expected `{}`, got `{}`.",
          CONSUMED,
          output
        );
      }
      other => panic!(
        "Parser `take_while1` didn't succeed when it should have. \
         Got `{:?}`.",
        other
      ),
    };
  }

  #[test]
  fn take_while1_set_succeed_str() {
    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const MATCH: &str = "βèƒôřèÂßÇ";
    const CONSUMED: &str = "βèƒôřèÂßÇ";
    const LEFTOVER: &str = "áƒƭèř";
    fn test(input: &str) -> IResult<&str, &str> {
      take_while1(MATCH)(input)
    }
    match test(INPUT) {
      Ok((extra, output)) => {
        assert!(
          extra == LEFTOVER,
          "Parser `is_a` consumed leftover input. Leftover `{}`.",
          extra
        );
        assert!(
          output == CONSUMED,
          "Parser `is_a` doesn't return the string it consumed on success. Expected `{}`, got `{}`.",
          CONSUMED,
          output
        );
      }
      other => panic!(
        "Parser `is_a` didn't succeed when it should have. \
         Got `{:?}`.",
        other
      ),
    };
  }

  #[test]
  fn take_while1_fn_fail_str() {
    use winnow::bytes::take_while1;

    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    fn while1_s(c: char) -> bool {
      c == '9'
    }
    fn test(input: &str) -> IResult<&str, &str> {
      take_while1(while1_s)(input)
    }
    match test(INPUT) {
      Err(ErrMode::Error(_)) => (),
      other => panic!(
        "Parser `take_while1` didn't fail when it should have. \
         Got `{:?}`.",
        other
      ),
    };
  }

  #[test]
  fn take_while1_set_fail_str() {
    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const MATCH: &str = "Ûñℓúçƙ¥";
    fn test(input: &str) -> IResult<&str, &str> {
      take_while1(MATCH)(input)
    }
    match test(INPUT) {
      Err(ErrMode::Error(_)) => (),
      other => panic!(
        "Parser `is_a` didn't fail when it should have. Got `{:?}`.",
        other
      ),
    };
  }

  #[test]
  fn take_till_succeed_str() {
    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const CONSUMED: &str = "βèƒôřèÂßÇ";
    const LEFTOVER: &str = "áƒƭèř";
    fn till_s(c: char) -> bool {
      c == 'á'
    }
    fn test(input: &str) -> IResult<&str, &str> {
      take_till(till_s)(input)
    }
    match test(INPUT) {
      Ok((extra, output)) => {
        assert!(
          extra == LEFTOVER,
          "Parser `take_till` consumed leftover input."
        );
        assert!(
          output == CONSUMED,
          "Parser `take_till` doesn't return the string it consumed on success. \
           Expected `{}`, got `{}`.",
          CONSUMED,
          output
        );
      }
      other => panic!(
        "Parser `take_till` didn't succeed when it should have. \
         Got `{:?}`.",
        other
      ),
    };
  }

  #[test]
  fn take_till1_succeed_str() {
    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const AVOID: &str = "£úçƙ¥á";
    const CONSUMED: &str = "βèƒôřèÂßÇ";
    const LEFTOVER: &str = "áƒƭèř";
    fn test(input: &str) -> IResult<&str, &str> {
      take_till1(AVOID)(input)
    }
    match test(INPUT) {
      Ok((extra, output)) => {
        assert!(
          extra == LEFTOVER,
          "Parser `take_till1` consumed leftover input. Leftover `{}`.",
          extra
        );
        assert!(
          output == CONSUMED,
          "Parser `take_till1` doesn't return the string it consumed on success. Expected `{}`, got `{}`.",
          CONSUMED,
          output
        );
      }
      other => panic!(
        "Parser `take_till1` didn't succeed when it should have. \
         Got `{:?}`.",
        other
      ),
    };
  }

  #[test]
  fn take_till1_failed_str() {
    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const AVOID: &str = "βúçƙ¥";
    fn test(input: &str) -> IResult<&str, &str> {
      take_till1(AVOID)(input)
    }
    match test(INPUT) {
      Err(ErrMode::Error(_)) => (),
      other => panic!(
        "Parser `is_not` didn't fail when it should have. Got `{:?}`.",
        other
      ),
    };
  }

  #[test]
  #[cfg(feature = "alloc")]
  fn recognize_is_a_str() {
    let a = "aabbab";
    let b = "ababcd";

    fn f(i: &str) -> IResult<&str, &str> {
      many1(alt((tag("a"), tag("b")))).recognize().parse_next(i)
    }

    assert_eq!(f(a), Ok((&a[6..], a)));
    assert_eq!(f(b), Ok((&b[4..], &b[..4])));
  }

  #[test]
  fn utf8_indexing_str() {
    fn dot(i: &str) -> IResult<&str, &str> {
      tag(".")(i)
    }

    let _ = dot("點");
  }
}
