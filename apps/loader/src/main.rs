#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

const PLASH_START: usize = 0x22000000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let pl_start = PLASH_START as *const u8;
    let apps_num = unsafe {core::slice::from_raw_parts(pl_start, 1)[0] };
        //32; // Dangerous!!! We need to get accurate size of apps.

    
    println!("app_num:{}",apps_num);
    println!("Load payload ...");
    let mut now_pos=PLASH_START+1;
    for i in 0..apps_num {
        let app_size=unsafe {
            ((core::slice::from_raw_parts(now_pos as *const u8, 1)[0] as usize)<<8)+
                (core::slice::from_raw_parts((now_pos+1)as *const u8, 1)[0] as usize)
        };
        let code=unsafe {
            core::slice::from_raw_parts((now_pos+2)as *const u8, app_size)
        };
        println!("app{}\nsize:{} \ncontent:{:?}", i,app_size,code);
        now_pos+=app_size+2;
    }
    println!("Load payload ok!");
}