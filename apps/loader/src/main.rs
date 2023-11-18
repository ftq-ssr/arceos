#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(asm_const)]
#[cfg(feature = "axstd")]
use axstd::println;

const PLASH_START: usize = 0x22000000;
const RUN_START: usize = 0xffff_ffc0_8010_0000;
#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let apps_num = unsafe {core::slice::from_raw_parts(PLASH_START as *const u8, 1)[0] };
    println!("app_num:{}",apps_num);
    let mut now_pos=PLASH_START+1;
    for i in 0..apps_num {
        println!("Load payload ...");
        let app_size=unsafe {
            ((core::slice::from_raw_parts(now_pos as *const u8, 1)[0] as usize)<<8)+
                (core::slice::from_raw_parts((now_pos+1)as *const u8, 1)[0] as usize)
        };
        let code=unsafe {
            core::slice::from_raw_parts((now_pos+2)as *const u8, app_size)
        };
        println!("app{} size:{} \ncontent:{:?}", i,app_size,code);
        now_pos+=app_size+2;
        // app running aspace
        // SBI(0x80000000) -> App <- Kernel(0x80200000)
        // 0xffff_ffc0_0000_0000
        let run_code = unsafe {
            core::slice::from_raw_parts_mut(RUN_START as *mut u8, app_size)
        };
        run_code.copy_from_slice(code);
        println!("run code {:?}; address [{:?}]", run_code, run_code.as_ptr());
        println!("Load payload ok!");

        println!("Execute app ...");
        // execute app
        unsafe { core::arch::asm!("
            li      t2, {run_start}
            jalr    t2",
            run_start = const RUN_START,
        )}
        println!();
    }
    
}