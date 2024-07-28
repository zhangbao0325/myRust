//https://www.bilibili.com/video/BV1uxaceoE8Q/?spm_id_from=333.1007.tianma.2-3-6.click&vd_source=33c855b334d41d666d1dffd84af69cef

struct BigStruct {
    data: [u8; 100000000000],
}

fn main() {
    let x = BigStruct {
        data: [0_u8; 100000000000],
    };

    let _x_in_heap = Box::new(x);

    println!("Hello, world!");
}

// cargo run debug --> stack overflow
// cargo run release --> ok
