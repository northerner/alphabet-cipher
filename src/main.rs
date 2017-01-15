#[test]
fn it_works() {
    assert_eq!(encode_message("meetmebythetree", "scones"), "egsgqwtahuiljgs");
}

fn main() {
    encode_message("meetmebythetree", "scones");
}

fn encode_message(message: &str, keyword: &str) -> String {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let table = (0..26)
        .map(|i| (alphabet[i..26].to_string() + &alphabet[0..i]))
        .collect::<Vec<String>>();

    let mut keyword_cycle = keyword.chars().cycle();
    let mut encoded_message = "".to_string();
    for mc in message.chars() {
        let key = keyword_cycle.next().unwrap();
        let col = alphabet.find(key).unwrap();
        let row = alphabet.find(mc).unwrap();
        encoded_message.push(table[row].chars().nth(col).unwrap());
    }
    encoded_message
}
