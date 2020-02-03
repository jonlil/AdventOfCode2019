use std::collections::BTreeMap;

const INPUT_MIN: u32 = 245318;
const INPUT_MAX: u32 = 765747;

fn digits(value: u32) -> Vec<u32> {
    value.to_string()
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect()
}

fn main() {
    let data: Vec<u32> = (INPUT_MIN..=INPUT_MAX)
        .into_iter()
        .filter(|value| {
            let mut iterator = digits(*value).into_iter();
            let mut a: Option<u32> = iterator.next();
            let mut b: Option<u32> = None;
            let mut is_decreasing: bool = false;
            let mut has_double: bool = false;
            let mut map: BTreeMap<u32, u32> = BTreeMap::new();

            loop {
                b = match iterator.next() {
                    None => break,
                    b => b,
                };

                if b < a {
                    is_decreasing = true;
                    break;
                }

                if b == a {
                    has_double = true;
                    if let Some(x) = map.get_mut(&b.unwrap()) {
                        *x += 1;
                    } else {
                        map.insert(b.unwrap(), 2);
                    }
                }

                a = b;
            }

            if is_decreasing == true || has_double == false {
                false
            } else {
                let mut values: Vec<u32> = map.values().cloned().collect();
                values.sort();

                if values[0] == 2 {
                    true
                } else {
                    false
                }
            }
        })
        .collect();

    eprintln!("{:#?}", data.len());
}
