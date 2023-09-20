use std::borrow::Cow;
use std::pin::pin;

use generatox::prelude::*;

#[test]
fn test() {
    let range = pin!(range(0, 10));

    for i in range.yields() {
        println!("{i}")
    }
}

generator! {
    pub fn range(start: i32, end: i32) yield i32 {
        for i in start..end {
            yield i;
        }
    }
}

generator! {
    pub fn entropy_by_depth<'a>(data: Cow<'a, [u8]>, depth: usize) yield f32 {
        if depth % 2 != 0 {
            panic!("depth must be an even number")
        }

        for i in 0..data.len() {
            let start = i.checked_sub(depth).unwrap_or_default();
            let end = data.len().min(i + depth + 1);

            let needle = data[i];
            let left = &data[start..i];
            let right = &data[i + 1..end];

            let count = 1 + count(needle, left) + count(needle, right);
            let prob = (count as f32) / (left.len() + right.len()) as f32;

            yield -f32::log2(prob);
        }
    }
}

generator! {
    fn parse_selector_segment<'a>(
        input: &'a str,
        f: &'_ mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result yield &'a str
    {
        f.write_str(input)?;
        yield input;
        return Ok(())
    }
}

/*
fn parse_selector_segment2<'a   : '__0, 'b   : '__0, '__0>(input: &'a str, f: &'__0 mut std::fmt::Formatter<'b>) -> impl '__0 + ::generatox::Generator<Yield=&'a str, Return=std::fmt::Result> {
    let f = f as &'__0 mut std::fmt::Formatter<'__0>;
    ::generatox::wrapper::Wrapper::new(async move {
        f.write_str(input)?;
        unsafe { ::generatox::wrapper::r#yield::<&'a str>(input) }.await;
        return Ok(());
    })
}
*/

fn count(needle: u8, mut haystack: &[u8]) -> usize {
    todo!()
}
