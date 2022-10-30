pub fn splitString(srcStr:&str, splitStr:&str) -> Vec<String> {
    let strList : Vec<String> = srcStr.split(splitStr).into_iter()
    .map(move |tempStr| tempStr.to_string()).collect();
    strList
}

