use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn player_move(start: i32, player: i32, list: &mut [i32]) -> i32 {
    let mut choice = start;
    let ban;
    if player == 1 {
        ban = 13;
    } else {
        ban = 6;
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
pub fn eatable(last: i32, player: i32, list: &[i32]) -> bool {
    if player == 1 {
        if last >= 0 && last <= 5 && list[last as usize] == 1 && list[(12 - last) as usize] != 0 {
            return true;
        }
    } else {
        if last >= 7 && last <= 12 && list[last as usize] == 1 && list[(12 - last) as usize] != 0 {
            return true;
        }
    }
    return false;
}

#[wasm_bindgen]
pub fn calculate_next(last: i32, player: i32, list: &mut [i32]) -> i32 {
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
pub fn cal_score(flag: i32, status: &[i32], step: i32, origin_flag: i32) -> i32{
    let point_hole;
    if origin_flag == 1 {
        point_hole = 6;
    } else {
        point_hole = 13;
    }
    let sum1: i32 = status[0..6].iter().sum();
    let sum2: i32 = status[7..13].iter().sum();
    let point = status[point_hole as usize] - status[(19 - point_hole) as usize];
    if step == 0 || sum1 == 0 || sum2 == 0 {
        return point;
    }
    let mut best_hole = prior(flag, status);
    if best_hole == -1 {
        best_hole = search(flag, status, step - 1);
    } 
    let mut next_status = [0;14];
    for i in 0..14 {
        next_status[i] = status[i];
    }
    let final_pos = player_move(best_hole, flag, &mut next_status);
    let ought = calculate_next(final_pos, flag, &mut next_status);
    if ought == flag {
        return cal_score(ought, &next_status, step - 1, origin_flag);
    } else {
        return cal_score(ought, &next_status, step - 1, origin_flag);
    }
}

#[wasm_bindgen]
pub fn search(flag: i32, status: &[i32], step: i32) -> i32 {
    let end;
    if flag == 1 {
        end = 5;
    } else {
        end = 12;
    }
    let mut best_hole = -1;
    let mut best_score = -100;
    for i in 0..6 {
        let hole = end - i;
        if status[hole as usize] == 0 {
            continue;
        }
        let mut next_status = [0;14];
        for i in 0..14 {
            next_status[i] = status[i];
        }
        let final_pos = player_move(hole, flag, &mut next_status);
        let ought = calculate_next(final_pos, flag, &mut next_status);
        let score = cal_score(ought, &next_status, step, flag);
        if score > best_score {
            best_hole = hole;
            best_score = score;
        }
    }
    return best_hole;
}

#[wasm_bindgen]
pub fn prior(flag: i32, status: &[i32]) -> i32{
    let end;
    if flag == 1 {
        end = 5;
    } else {
        end = 12;
    }
    // 2 priorior strategies --  continue moving(1), eat(2); else choose best from holes with most.
    for i in 0..6 {
        let hole = end - i;
        if status[hole as usize] == 0 {
            continue;
        }
        let mut next_status = [0;14];
        for i in 0..14 {
            next_status[i] = status[i];
        }
        let final_pos = player_move(hole, flag, &mut next_status);
        if (flag == 1 && final_pos == 6) || (flag == 2 && final_pos == 13) || eatable(final_pos, flag, &next_status) { // continue moving
            return hole;
        }
    }
    return -1;
}

#[wasm_bindgen]
pub fn mancala_operator(flag: i32, status: &[i32]) -> i32 {
    let mut hole:i32 = prior(flag,status);
    if hole == -1 {
        hole = search(flag, status, 10); // step to adjust the depth of DFS
    }
    return flag * 10 + hole + 1 - 7 * (flag - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testone() {
    }
}