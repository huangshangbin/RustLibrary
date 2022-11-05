pub fn splitString(srcStr:&str, splitStr:&str) -> Vec<String> {
    let strList : Vec<String> = srcStr.split(splitStr).into_iter()
    .map(move |tempStr| tempStr.to_string()).collect();
    strList
}

pub fn splitStringGetOneStr(srcStr:&str, splitStr:&str, index:usize) -> String {
    let strList : Vec<String> = srcStr.split(splitStr).into_iter()
    .map(move |tempStr| tempStr.to_string()).collect();
    strList[index].clone()
}

pub fn replaceString(srcStr : &str, replaceStr : &str, targetStr : &str) -> String {
    srcStr.replace(replaceStr, targetStr).to_string()
}

pub fn isExistStringInString(srcStr : &str, existStr : &str) -> bool {
    srcStr.contains(existStr)
}

pub fn isUseStringEnd(srcStr : &str, endStr : &str) -> bool {
    srcStr.ends_with(endStr)
}

pub fn getStringUsePos(srcStr : &str, startPos : usize, endPos : usize) -> String {
	srcStr[startPos..endPos + 1].to_string()
}

pub fn getStringUseCharStart(srcStr : &str, startChar : char) -> String {
    let mut index : usize = 0;
    for i in srcStr.chars() {
        if i == startChar {
            break;
        }
        index = index + 1;
    }
    srcStr[index..srcStr.len()].to_string()
}

pub fn getStringUseCharEnd(srcStr : &str, endChar : char) -> String {
    let mut index : usize = 0;
    for i in srcStr.chars() {
        if i == endChar {
            break;
        }
        index = index + 1;
    }
    srcStr[0..index].to_string()
}