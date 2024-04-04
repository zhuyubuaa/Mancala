use wasm_bindgen::prelude::*;
use js_sys::Int32Array;

#[wasm_bindgen]
pub fn player_move(op: i32,player: i32,list: &mut [i32]) -> Int32Array {
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
pub fn calculate_next(last: i32,player: i32,list: &mut [i32]) -> i32 {
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
pub fn mancala_board(flag: i32, seq: &[i32], size: i32) -> i32 {
    let mut list = [4;14];
    list[6] = 0;
    list[13] = 0;
    let mut ought = flag;
    let mut index = -1;
    for op in seq {
        index = index + 1;
        let player = op / 10;
        let last = player_move(*op, player, &mut list);
        ought = calculate_next(last, player, &mut list);
    }
    let sum1: i32 = list[0..6].iter().sum();
    let sum2: i32 = list[7..13].iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;
}