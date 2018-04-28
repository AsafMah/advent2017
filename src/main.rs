fn main() {
    let input: &'static str = include_str!("input_day_9");
    let mut stack = Vec::new();
    let mut escape = false;
    let mut garbage = false;
    let mut score = 0;
    let mut count = 0;

    for c in input.chars() {
        match c {
            _ if escape => {
                escape = false;
            },
            '!' => {
                escape = true;
            },
            '>' => {
                stack.pop();
                garbage = false;
            },
            _ if garbage => {
                count += 1;
            },
            '{' => {
                stack.push('{');
                score += stack.len();
            },
            '}' => {
                stack.pop();
            },
            '<' => {
                stack.push('<');
                garbage = true;
            },
            _ => {}
        }
    }

    eprintln!("score = {:?}, count = {:?}", score, count);
}