use library::utils::StringUtils;
fn main() {
    let strList = StringUtils::splitString("huangshangbin", "a");
    for str in strList {
        println!("{}", str);
    }
}
