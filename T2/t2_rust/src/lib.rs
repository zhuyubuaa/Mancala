use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn player_move(op: i32,player: i32,list: &mut [i32]) -> i32 {
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
    if rest == 0 {
        return -1;
    }
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
pub fn mancala_result(flag: i32, seq: &[i32], size: i32) -> i32 {
    let mut list = [4;14];
    list[6] = 0;
    list[13] = 0;
    let mut ought = flag;
    let mut index = -1;
    for op in seq {
        index = index + 1;
        let sum1: i32 = list[0..6].iter().sum();
        let sum2: i32 = list[7..13].iter().sum();
        let player = op / 10;
        if sum1 == 0 || sum2 == 0 {
            return 30000 + index;
        }
        if ought != player {
            return 30000 + index;
        } 
        let last = player_move(*op, player, &mut list);
        if last  == -1 {
            return 30000 + index;
        }
        ought = calculate_next(last,player,&mut list);
    }
    let sum1: i32 = list[0..6].iter().sum();
    let sum2: i32 = list[7..13].iter().sum();
    if sum1 == 0 || sum2 == 0 {
        list[6] = list[6] + sum1;
        list[13] = list[13] + sum2;
        if flag == 1 {
            return 15000 + list[6] - list[13];
        } else {
            return 15000 + list[13] - list[6];
        }
    } else {
        if flag == 1 {
            return 20000 + list[6];
        } else {
            return 20000 + list[13];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testone() {
        assert_eq!(mancala_result(1,&[11,12],2),30001);
    }

    #[test]
    fn testtwo() {
        assert_eq!(mancala_result(1,&[14],1),20001);
    }
}