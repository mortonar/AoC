use anyhow::Result;

fn main() -> Result<()> {
    let mut password = String::new();
    std::io::stdin().read_line(&mut password)?;
    let mut password: Vec<char> = password.trim().chars().collect();

    while !validate(&password) {
        increment_pass(&mut password);
    }
    println!("Part 1: {}", password.iter().collect::<String>());

    increment_pass(&mut password);
    while !validate(&password) {
        increment_pass(&mut password);
    }
    println!("Part 2: {}", password.iter().collect::<String>());

    Ok(())
}

fn validate(password: &[char]) -> bool {
    let straight = password
        .windows(3)
        .any(|w| w[1] == increment(w[0]) && w[2] == increment(w[1]));
    if !straight {
        return false;
    }

    let forbidden_chars = ['i', 'o', 'l'];
    if password.iter().any(|c| forbidden_chars.contains(c)) {
        return false;
    }

    let mut has_pair = false;
    for i in 0..password.len() - 3 {
        for j in i + 2..password.len() - 1 {
            if password[i] == password[i + 1] && password[j] == password[j + 1] {
                has_pair = true;
                break;
            }
        }
    }
    has_pair
}

fn increment_pass(password: &mut [char]) {
    for c in password.iter_mut().rev() {
        if *c == 'z' {
            *c = 'a';
        } else {
            *c = increment(*c);
            break;
        }
    }
}

fn increment(c: char) -> char {
    (u8::try_from(c).unwrap() + 1) as char
}

#[test]
fn test_increment() {
    let mut password = vec!['a', 'b', 'c', 'd', 'e'];
    increment_pass(&mut password);
    assert_eq!(password, vec!['a', 'b', 'c', 'd', 'f']);

    let mut password = vec!['a', 'z', 'z', 'z', 'z'];
    increment_pass(&mut password);
    assert_eq!(password, vec!['b', 'a', 'a', 'a', 'a']);
}

#[test]
fn test_validate() {
    let password: Vec<char> = "hijklmmn".chars().collect();
    assert!(!validate(&password));
    let password: Vec<char> = "abbceffg".chars().collect();
    assert!(!validate(&password));
    let password: Vec<char> = "abbcegjk".chars().collect();
    assert!(!validate(&password));
}
