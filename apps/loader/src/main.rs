#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(asm_const)]
#[cfg(feature = "axstd")]
use axstd::{println,process::exit};

const PLASH_START: usize = 0x22000000;
// const RUN_START: usize = 0xffff_ffc0_8010_0000;
const RUN_START: usize = 0x4010_0000;
const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;

static mut ABI_TABLE: [usize; 16] = [0; 16];

fn register_abi(num: usize, handle: usize) {
    unsafe { ABI_TABLE[num] = handle; }
}

fn abi_hello() {
    println!("[ABI:Hello] Hello, Apps!");
}

fn abi_putchar(c: char) {
    println!("[ABI:Print] {c}");
}

fn abi_terminate() {
    println!("[ABI:Terminate] Exit Code 0");
    exit(0);
}
#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let apps_num = unsafe {core::slice::from_raw_parts(PLASH_START as *const u8, 1)[0] };
    println!("app_num:{}",apps_num);
    let mut now_pos=PLASH_START+1;
    for i in 0..apps_num {
        unsafe { init_app_page_table(); }
        unsafe { switch_app_aspace(); }
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

        register_abi(SYS_HELLO, abi_hello as usize);
        register_abi(SYS_PUTCHAR, abi_putchar as usize);
        register_abi(SYS_TERMINATE, abi_terminate as usize);
        println!("Execute app ...");

        // execute app
        unsafe { core::arch::asm!("
            la      a0, {abi_table}
            li      t2, {run_start}
            jalr    t2",
            run_start = const RUN_START,
            abi_table = sym ABI_TABLE,
        )}
        println!();
    }
    
}

//
// App aspace
//

#[link_section = ".data.app_page_table"]
static mut APP_PT_SV39: [u64; 512] = [0; 512];

unsafe fn init_app_page_table() {
    // 0x8000_0000..0xc000_0000, VRWX_GAD, 1G block
    APP_PT_SV39[2] = (0x80000 << 10) | 0xef;
    // 0xffff_ffc0_8000_0000..0xffff_ffc0_c000_0000, VRWX_GAD, 1G block
    APP_PT_SV39[0x102] = (0x80000 << 10) | 0xef;

    // 0x0000_0000..0x4000_0000, VRWX_GAD, 1G block
    APP_PT_SV39[0] = (0x00000 << 10) | 0xef;

    // For App aspace!
    // 0x4000_0000..0x8000_0000, VRWX_GAD, 1G block
    APP_PT_SV39[1] = (0x80000 << 10) | 0xef;
}

unsafe fn switch_app_aspace() {
    use riscv::register::satp;
    let page_table_root = APP_PT_SV39.as_ptr() as usize - axconfig::PHYS_VIRT_OFFSET;
    satp::set(satp::Mode::Sv39, 0, page_table_root >> 12);
    riscv::asm::sfence_vma_all();
}