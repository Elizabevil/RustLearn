pub const reset: &str = "0";
pub const bold: &str = "1";
pub const slope: &str = "3";
pub const underline: &str = "4";
pub const flashing: &str = "5";
pub const hide: &str = "8";

const start: &str = "\x1B[";
const end: &str = "\x1B[0m";



impl Color {
    fn area(&self) {
        let list_of_numbers = vec![1, 2, 3];
        let list_of_strings: Vec<String> = list_of_numbers
            .iter()
            .map(|i| i.to_string())
            .collect();
    }

    pub fn color(front_color: i32, back_color: i32, color_type: i32, color_type2: i32) {
        print!("\x1B[{front_color};{back_color};{color_type};{color_type2}mMCC\x1B[0m");
        // "\x1B[{front_color};{back_color};{color_type};{color_type2}mMCC\x1B[0m"
    }

    pub fn color2(color_types: &[i32], target_str: &str) {
        // print!("{}{}m{}{}", start, color_types.join(";"), target_str, end);
    }

    pub fn Yellow(&self, color_types: &[i32], target_str: &str) {
        // print!("{}{}m{}{}", start, color_types.join(";"), target_str, end);
    }

    pub fn Print() {}
}
 