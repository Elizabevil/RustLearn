trait KtStd {
    fn also_ref(&self, block: impl FnOnce(&Self)) -> &Self {
        block(self);
        self
    }
}

// 任意类型都可点出该函数
impl<T> KtStd for T {}

struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
