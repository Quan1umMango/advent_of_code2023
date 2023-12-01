fn part_1(input_string:&str) -> String {
    let mut out_string = "".to_string();
    for ch in input_string.chars() {
        if ch.is_digit(10) {
            out_string = format!("{}{}",ch,out_string);
            break;
        }
    }

    for ch in input_string.chars().rev(){
        if ch.is_digit(10) {
            out_string = format!("{}{}",out_string,ch);
            break;
        }
    }
    return out_string;
}


fn main() { 
    let input_string = String::from_utf8_lossy(include_bytes!("./input.txt"));
    let mut fin = 0;
    for line in input_string.lines() {
        if let Some(val) = part_1(line).parse::<u32>().ok() {
            println!("{}",val);
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
        assert_eq!("42".to_string(),part_1("dsakdha4najdqo2") );
    }
}
