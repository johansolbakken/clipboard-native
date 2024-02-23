extern "C" {
    fn platform_get_clipboard_string();
    fn platform_set_clipboard_string(s: *const i8);
}

pub fn set_clipboard_content(contents: &String) {
    let c_str = std::ffi::CString::new(contents.as_str()).unwrap();
    let s = c_str.as_ptr();
    unsafe {
        platform_set_clipboard_string(s);
    }
}

pub fn get_clipboard_content() -> String {
    unsafe {
        platform_get_clipboard_string();
    }
    let contents = std::fs::read_to_string("clipboard.txt").unwrap();
    std::fs::remove_file("clipboard.txt").unwrap();
    contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set_clipboard() {
        let old = get_clipboard_content();
        let new = "Hello, World!".to_string();
        set_clipboard_content(&new);
        let s = get_clipboard_content();
        assert_eq!(s, new);
        set_clipboard_content(&old);
    }
}