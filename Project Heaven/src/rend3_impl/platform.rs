#![allow(non_snake_case, unused)]

cfg_if::cfg_if!(
    if #[cfg(target_os = "macos")] {
        // https://stackoverflow.com/a/16125341 reference
        pub mod Scancodes {
            pub const W: u32 = 0x0D;
            pub const A: u32 = 0x00;
            pub const S: u32 = 0x01;
            pub const D: u32 = 0x02;
            pub const Q: u32 = 0x0C;
            pub const Z: u32 = 0x06;
            pub const P: u32 = 0x23;
            pub const SEMICOLON: u32 = 0x29;
            pub const QUOTE: u32 = 0x27;
            pub const COMMA: u32 = 0x2B;
            pub const PERIOD: u32 = 0x2F;
            pub const SHIFT: u32 = 0x38;
            pub const ESCAPE: u32 = 0x35;
            pub const LALT: u32 = 0x3A; // Actually Left Option
            pub const SPACE: u32 = 0x31;
            pub const CTRL: u32 = 0x3B;
            pub const E: u32 = 0x0E;
            pub const PLUS_NUM: u32 = 0x45;
            pub const MINUS_NUM: u32 = 0x4E;
            pub const UP: u32 = 0x5B;
            pub const DOWN: u32 = 0x54;
            pub const LEFT: u32 = 0x56;
            pub const RIGHT: u32 = 0x58;
            pub const I: u32 = 0x22;
            pub const K: u32 = 0x28;
            pub const J: u32 = 0x26;
            pub const L: u32 = 0x25;
        }
    } else { /*https://www.win.tue.nl/~aeb/linux/kbd/scancodes-1.html*/
        pub mod Scancodes {
            pub const W: u32 = 0x11;
            pub const A: u32 = 0x1E;
            pub const S: u32 = 0x1F;
            pub const D: u32 = 0x20;
            pub const Q: u32 = 0x10;
            pub const Z: u32 = 0x2C;
            pub const P: u32 = 0x19;
            pub const SEMICOLON: u32 = 0x27;
            pub const QUOTE: u32 = 0x28;
            pub const COMMA: u32 = 0x33;
            pub const PERIOD: u32 = 0x34;
            pub const SHIFT: u32 = 0x2A;
            pub const ESCAPE: u32 = 0x01;
            pub const LALT: u32 = 0x38;
            pub const SPACE: u32 = 0x39;
            pub const CTRL: u32 = 0x1D;
            pub const E: u32 = 0x12;
            pub const PLUS_NUM: u32 = 0x4e;
            pub const MINUS_NUM: u32 = 0x4a;
            pub const UP: u32 = 0x48;
            pub const DOWN: u32 = 0x50;
            pub const LEFT: u32 = 0x4b;
            pub const RIGHT: u32 = 0x4d;
            pub const I: u32 = 0x17;
            pub const K: u32 = 0x25;
            pub const J: u32 = 0x24;
            pub const L: u32 = 0x26;
        }
    }
);
