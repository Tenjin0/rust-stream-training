use std::collections::HashMap;

fn main() {
    // faust.txt must be a UTF16 BE file
    let contents = std::fs::read("./faust.txt").unwrap();
    let contents : Vec<u16>= contents.chunks(2)
    .map(|bytes| {
        if bytes.len() == 2 {
            let  [first, second] = [bytes[0], bytes[1]];
            (first as u16) << 8 | (second as u16)
        } else {
            bytes[0] as u16
        }

    }).collect();

    let contents = String::from_utf16(&contents[..]).unwrap();
    println!("{}", contents);
}


fn process(string: String) -> HashMap<String, Vec<String>> {
    // let hash: HashMap<String, Vec<String>> = HashMap::new();
    
    for line in string.lines() {
        
    }
    // return hash;
    todo!()
}
