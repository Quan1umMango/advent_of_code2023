fn remove_id(s:&mut String)  {
    let mut new_s = s.clone();
    for ch in s.chars() {
        if ch.is_digit(10) {
            new_s =new_s.strip_prefix(ch).unwrap().to_string();
        }else if ch == ':' {
            new_s = new_s.strip_prefix(":").unwrap().to_string();
            break;
        }
    }
    *s = new_s;}

fn get_individual_games(s:String) -> Vec<String> {
    let mut cur_game = "".to_string();
    let mut out = Vec::new();

    for i in 0..s.len() {
        let ch = s.chars().nth(i).unwrap();
        if i == s.len()-1 {
            cur_game += ch.to_string().as_str();
            out.push(cur_game.clone());
        }
        if ch == ',' {
            continue;
        } 
        if ch.is_digit(10) || ch.is_alphabetic() {
            cur_game += ch.to_string().as_str();
            continue;
        }
        if ch == ';' {
            out.push(cur_game.clone());
            cur_game = "".to_string();
        }    

    }
    return out;
}

fn eval_game_get_max(games:Vec<String>) -> u128 {

    let mut red_min = 0;
    let mut blue_min = 0;
    let mut green_min = 0;

    for subgame in games.iter() {

        let mut i = 0;
        let mut temp_string = "".to_string();

        while i < subgame.len() {
            let ch = subgame.chars().nth(i).unwrap();
            if ch.is_digit(10) {
                temp_string.push(ch);
                i += 1;
                continue;
            }
            if ch == 'g' {
                let num =  temp_string.parse::<u32>().ok().unwrap();
                if num> green_min {
                    green_min = num;
                }
                i += 5;
                temp_string.clear();
                continue;
            }
            if ch == 'r' {
                let num =  temp_string.parse::<u32>().ok().unwrap();
                if num> red_min {
                    red_min = num;
                }
                i += 3;
                temp_string.clear();
                continue;
            }
            if ch == 'b' {
                let num =  temp_string.parse::<u32>().ok().unwrap();
                if num> blue_min {
                    blue_min = num;
                }
                i += 4;
                temp_string.clear();
                continue;
            }
            i+=1;
        }

    }
    return (red_min as u64 * blue_min as u64* green_min as u64).into();
}


fn main() {
    let input = String::from_utf8_lossy(include_bytes!("./test_input.txt"));
    let mut out = 0;
    for line in input.lines() {
        let mut l = line.strip_prefix("Game ").unwrap().to_string();
     remove_id(&mut l); 
        let games = get_individual_games(l);
        out += eval_game_get_max(games);
        
    }
    println!("{}",out);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
           let input = String::from_utf8_lossy(include_bytes!("./test_input.txt"));
    let mut out = 0;
    for line in input.lines() {
        let mut l = line.strip_prefix("Game ").unwrap().to_string();
    remove_id(&mut l);
        let games = get_individual_games(l);
        out += eval_game_get_max(games);
        
    }        assert_eq!(out,2286);

    }
}
