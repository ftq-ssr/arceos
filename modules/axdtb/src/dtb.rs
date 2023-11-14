use core::fmt::Error;
use alloc::vec::Vec;

#[allow(unused_imports)]
use axlog;

// CONSTANTS
const DTB_MAGIC: u32 = 0xD00DFEED;
const DTB_VERSION: u32 = 17;

#[allow(dead_code)]
struct DtbHeader {
    magic: u32,
    totalsize: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32,
}

pub struct DtbInfo {
    pub memory_addr: usize,
    pub memory_size: usize,
    pub mmio_regions: Vec<(usize, usize)>,
}

fn check_header(header: &DtbHeader) -> bool {
    u32::from_be(header.magic) == DTB_MAGIC && u32::from_be(header.version) == DTB_VERSION
}
// pub unsafe fn from_raw(address: *const u8) -> Option<Self> {
//     let header = &*(address as *const DtbHeader);
//     if !Self::check_header(header) {
//         return None;
//     }

//     let address = header as *const _ as usize + u32::from_be(header.off_dt_struct) as usize;
//     let length = u32::from_be(header.size_dt_struct) as usize;
//     let struct_slice = slice::from_raw_parts(address as *const u8, length);

//     let address = header as *const _ as usize + u32::from_be(header.off_dt_strings) as usize;
//     let length = u32::from_be(header.size_dt_strings) as usize;
//     let strings_slice = slice::from_raw_parts(address as *const u8, length);

//     Some(Self {
//         header,
//         struct_slice,
//         strings_slice,
//     })
// }

fn translate(a:&[u8],pos:usize)->usize{
    (a[pos] as usize) << 24 | (a[pos+1] as usize) << 16 | (a[pos+2] as usize) << 8 | (a[pos+3] as usize)
}
pub fn parse_dtb(dtb_pa: usize) -> Result<DtbInfo,Error> {
    unsafe {
        let address = dtb_pa as *const u8;
        let header = &*(address as *const DtbHeader);
        if !check_header(header) {return Err(Error);}
        let fdt = fdt::Fdt::from_ptr(dtb_pa as *const u8).unwrap();
        let addr = fdt.memory().regions().next().unwrap().starting_address as usize;
        if let Some(size) = fdt.memory().regions().next().unwrap().size {
            let soc = fdt.find_node("/soc");
            if let Some(soc) = soc {
                let mut regions = Vec::new();
                for child in soc.children() {
                    if child.name.contains("mmio") {
                        if let Some(reg_prop) = child.property("reg") {
                            regions.push((translate(reg_prop.value, 4),translate(reg_prop.value, 12)));
                        } else {return Err(Error);}
                    }
                }
                return Ok(DtbInfo {
                    memory_addr: addr,
                    memory_size: size,
                    mmio_regions: regions,
                });
            }
        }
        Err(Error)
    }
}