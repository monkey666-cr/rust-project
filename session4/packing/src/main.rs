struct OneByte {
    a: u8,
}

struct TwoBytes {
    a: u16,
}

// 通信场景下固定的传输数据使用packed, 如果时定义变量申请内容不要使用, 这样会让寻址变得很慢
#[repr(packed)]
struct ThreeBytes {
    a: u16,
    b: u8,
}

struct FourBytes {
    a: u32,
}

fn main() {
    println!("{}", std::mem::size_of::<OneByte>());
    println!("{}", std::mem::size_of::<TwoBytes>());
    println!("{}", std::mem::size_of::<ThreeBytes>());
    println!("{}", std::mem::size_of::<FourBytes>());
}
