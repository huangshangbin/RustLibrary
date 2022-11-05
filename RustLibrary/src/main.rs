use library::utils::StringUtils;
fn main() {
    //let srcStr = String::from("huangshangbin");
    //let subStr = &srcStr[0..6];

    //println!("subStr={}", subStr);

    let srcStr = String::from("huangshangbin");
    let splitStr = String::from("a");
    let result = StringUtils::getStringUseCharEnd(srcStr.as_str(), 'a');
    println!("result={}", result);
}
