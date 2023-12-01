const NUM_STRINGS:[&str;10] = ["zero","one","two","three","four","five","six","seven","eight","nine"];

fn find_left(input_str:&str) -> String {
    let mut cur = "".to_string();
    for ch in input_str.chars() {

        if ch.is_digit(10) {
            return ch.to_string();
        }else {
            cur =format!("{}{}",cur,ch) ;
        }
        for i in 0..NUM_STRINGS.len() {
            if cur.contains(NUM_STRINGS[i]) {
                return i.to_string();
            }
        }
    }
    return "".to_string();

}

fn find_right(input_str:&str) -> String {
    let mut cur = "".to_string();

    for ch in input_str.chars().rev() {
        if ch.is_digit(10) {
            return ch.to_string();
        }else {
            cur =format!("{}{}",ch,cur);
        }
        for i in 0..NUM_STRINGS.len() {
            if cur.contains(NUM_STRINGS[i]) {
                return i.to_string();
            }
        }
    }
    return "".to_string();
}

fn part_2(input_str:&str) -> String {
    let mut out = format!("{}{}",find_left(input_str),find_right(input_str));

    println!("{}",out);
    return out;
}

fn main() {
    let input_string = String::from_utf8_lossy(include_bytes!("./input.txt"));
    let mut fin = 0;
    for line in input_string.lines() {
        if let Some(val) = part_2(line).parse::<u32>().ok() {
            fin += val;
        }
    }
    println!("{}",fin);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        assert_eq!("92".to_string(),part_2("nine12") );
    }
}
