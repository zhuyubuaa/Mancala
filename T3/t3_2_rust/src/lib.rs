use wasm_bindgen::prelude::*;
use js_sys::Int32Array;

#[wasm_bindgen]
pub fn player_move(op: i32, player: i32,list: &mut [i32]) -> i32 {
    let mut choice = op % 10;
    let ban;
    if player == 1 {
        ban = 13;
        choice = choice - 1;
    } else {
        ban = 6;
        choice = choice + 6;
    }
    let mut rest = list[choice as usize];
    list[choice as usize] = 0;
    while rest != 0 {
        choice = choice + 1;
        if ban != choice % 14 {
            list[(choice % 14) as usize] = list[(choice % 14) as usize] + 1;
            rest = rest - 1;
        }
    }
    return choice % 14;
}

#[wasm_bindgen]
pub fn calculate_next(last: i32, player: i32,list: &mut [i32]) -> i32 {
    let ought;
    if player == 1 {
        if last == 6 {
            ought = 1;
        } else {
            if last >= 0 && last <= 5 && list[last as usize] == 1 && list[(12 - last) as usize] != 0 {
                list[6] = list[6] + list[last as usize] + list[(12 - last) as usize];
                list[last as usize] = 0;
                list[(12 - last) as usize] = 0;
            }
            ought = 2;
        }
    } else {
        if last == 13 {
            ought = 2;
        } else {
            if last >= 7 && last <= 12 && list[last as usize] == 1 && list[(12 - last) as usize] != 0 {
                list[13] = list[13] + list[last as usize] + list[(12 - last) as usize];
                list[last as usize] = 0;
                list[(12 - last) as usize] = 0;
            }
            ought = 1;
        }
    }
    return ought;
}

#[wasm_bindgen]
pub fn mancala_operater(flag: i32, status: &[i32]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testone() {
    }
}