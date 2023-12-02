
const MAX_GREEN:u32 = 13;
const MAX_RED:u32 =12;
const MAX_BLUE:u32 =14;

fn get_id(s:&mut String) -> u32 {
    let mut new_s = s.clone();
    let mut out_string = "".to_string();
    for ch in s.chars() {
        if ch.is_digit(10) {
            out_string += ch.to_string().as_str();
            new_s =new_s.strip_prefix(ch).unwrap().to_string();
        }else if ch == ':' {
            new_s = new_s.strip_prefix(":").unwrap().to_string();
            break;
        }
    }
    *s = new_s;
    return out_string.parse::<u32>().unwrap();
}

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

fn eval_game(games:Vec<String>) -> bool {


    for subgame in games.iter() {
        let mut i = 0;
        let mut temp_string = String::new();
        let mut green = 0;
        let mut blue = 0;
        let mut red = 0;


        while i < subgame.len() {
            let ch = subgame.chars().nth(i).unwrap();
            if ch.is_digit(10) {
                temp_string.push(ch);
                i += 1;
                continue;
            }

            if ch == 'g' {
                green +=  temp_string.parse::<u32>().ok().unwrap();
                i += 5;
                temp_string.clear(); 
                continue;
            }
            if ch == 'r' {
                red += temp_string.parse::<u32>().ok().unwrap();
                i += 3;
                temp_string.clear(); 
                continue;
            }
            if ch == 'b' {
                blue += temp_string.parse::<u32>().ok().unwrap();
                i += 4;
                temp_string.clear();                 continue;
            }

            i += 1;
        }
        if green > MAX_GREEN || red > MAX_RED || blue > MAX_BLUE {
            return false;
        }
    }


    return true;

}

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("./input.txt"));
    let mut out = 0;
    for line in input.lines() {
        let mut l = line.strip_prefix("Game ").unwrap().to_string();
        let id = get_id(&mut l);
        let games = get_individual_games(l);
        if eval_game(games) {
            out += id;
        }
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
        let id = get_id(&mut l);
        let games = get_individual_games(l);
        if eval_game(games) {
            out += id;
        }
    }
        assert_eq!(out,8);

    }
}
