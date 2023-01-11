pub mod log {

    pub const reset: i32 = 0;
    pub const bold: i32 = 1;
    pub const slope: i32 = 3;
    pub const underline: i32 = 4;
    pub const flashing: i32 = 5;
    pub const hide: i32 = 8;

    const start: &str = "\x1B[";
    const end: &str = "\x1B[0m";

    pub fn sd() {}

    pub fn color(front_color: i32, back_color: i32, color_type: i32, color_type2: i32) {
        print!("\x1B[{front_color};{back_color};{color_type};{color_type2}mMCC\x1B[0m");
        // "\x1B[{front_color};{back_color};{color_type};{color_type2}mMCC\x1B[0m"
    }

    pub fn color2(color_types: &[i32], target_str: &str) {
        // print!("{}{}m{}{}", start, color_types.join(";"), target_str, end);
    }
}
 