use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn player_move(start: i32, player: i32, list: &mut [i32]) -> i32 {
    // println!("{}", start);
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
        // print!("{} ", score);
        if score > best_score {
            best_hole = hole;
            best_score = score;
        }
        // if step == 0 {
        //     return status[(end + 1) as usize] - status[(18 - end) as usize];
        // } else {
        //     return search(ought, &next_status, step - 1);
        // }
    }
    // if best_hole == -1 {
        // for el in status {
        //     print!("{} ", el);
        // }
        // println!();
    //
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
    let pick : i32;

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
pub fn mancala_operater(flag: i32, status: &[i32]) -> i32 {
    // let end;
    // if flag == 1 {
    //     end = 5;
    // } else {
    //     end = 12;
    // }
    // let pick : i32;

    // // 2 priorior strategies --  continue moving(1), eat(2); else choose best from holes with most.
    // for i in 0..6 {
    //     let hole = end - i;
    //     if status[hole as usize] == 0 {
    //         continue;
    //     }
    //     let mut next_status = [0;14];
    //     for i in 0..14 {
    //         next_status[i] = status[i];
    //     }
    //     let final_pos = player_move(hole, flag, &mut next_status);
    //     if (flag == 1 && final_pos == 6) || (flag == 2 && final_pos == 13) || eatable(final_pos, flag, &next_status) { // continue moving
    //         return hole;
    //     }
    // }

    // let mut most_holes : Vec<i32> = Vec::new();
    // for i in 0..6 {
    //     let hole = end - i;
    //     if most_holes.is_empty() {
    //         most_holes.push(i);
    //     } else {
    //         if status[most_holes[0] as usize] < status[i as usize] {
    //             most_holes.clear();
    //         } else if status[most_holes[0] as usize] == status[i as usize]{
    //             most_holes.push(i);
    //         }
    //     }
    // }
    // if most_holes.len() == 1 {
    //     return most_holes[0];
    // } else if most_holes.len() > 1 {
        // return search(flag, status);

    let hole:i32 = prior(flag,status);
    if hole == -1 {
        return search(flag, status, 12);
    } else {
        return hole;
    }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testone() {
        println!("{}", mancala_operater(2, &[0,5,1,6,6,0,2,5,0,6,6,6,5,1]));
    }
}