use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn bocchi_shut_up(flag: i32, seq: &[i32], size: i32) -> i32 {
    let mut table = [0; 7];
    for el in seq {
        if el / 10 == flag {
            table[el % 10] = table[el % 10] + 1;
        }
    }

    let mut most_frq = 0;
    let mut same : bool = false;
    for el in table {
        if el > most_frq{
            same = false;
            most_frq = el;
        } else if el == most_frq {
            same = true;
        }
    }

    if same {
        10
    } else {
        flag * 10 + most_frq
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bocchi_shut_up_test_when_10() {
        assert_eq!(bocchi_shut_up(1, &[11,12,13], 3), 10);
    }

    #[test]
    fn bocchi_shut_up_test_when_not_10() {
        assert_eq!(bocchi_shut_up(1, &[11,12,12], 3), 12);
    }
}