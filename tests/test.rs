use generatox::prelude::*;

#[test]
fn test() {
    let range = range(0, 10);
    pin_mut!(range);

    for i in range.yields() {
        println!("{i}")
    }
}

generator! {
    pub fn entropy_by_depth(data: &[u8], depth: usize) yield f32 {
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

fn count(needle: u8, mut haystack: &[u8]) -> usize {
    let mut count = 0;
    while let Some(i) = memchr::memchr(needle, haystack) {
        haystack = &haystack[i + 1..];
        count += 1
    }

    return count;
}
