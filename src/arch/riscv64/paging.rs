use core::{arch::asm, fmt::Write};

use bitfield_struct::bitfield;

use super::{alloc::BumpAllocator, uart::Uart};

pub enum PagingMode {
    SV39 = 8,
}

#[bitfield(u64, clone = false)]
pub struct SatpRegister {
    #[bits(44)]
    ppn: u64,
    asid: u16,
    #[bits(4)]
    mode: u8,
}

#[bitfield(u64, clone = false)]
struct PTE39 {
    valid: bool,
    readable: bool,
    writable: bool,
    executable: bool,
    user: bool,
    global: bool,
    accessed: bool,
    dirty: bool,
    #[bits(2)]
    rsw: u8,
    #[bits(44)]
    ppn: u64,
    #[bits(10)]
    reserved: u16,
}

pub fn setup(alloc: &BumpAllocator, mode: &PagingMode, uart: &mut Uart) {
    let root_page = alloc.allocate_new_page();

    let kernel_start = 0x80000000 + 0x200000;
    let kernel_end = kernel_start + 0xA00000;
    // let kernel_end = kernel_start + 4096;
    for pa in (kernel_start..kernel_end).step_by(4096) {
        ident_map_new_page(uart, alloc, mode, root_page, pa);
    }

    // Map stack
    ident_map_new_page(uart, alloc, mode, root_page, 0x88000000 - 4096);

    // Map UART
    ident_map_new_page(uart, alloc, mode, root_page, 0x10000000);

    let _ = uart.write_str("====================\n");
    simulate_va(uart, root_page, 0x88000000 - 4096);

    enable_paging(uart, mode, root_page);

    let _ = uart.write_str("Paging enabled\n");
    let _ = uart.write_str("====================\n");
}

fn simulate_va(uart: &mut Uart, root_page: usize, va: usize) {
    unsafe {
        uart.debug_value("root_page", root_page as u64);
        uart.debug_value("va", va as u64);

        let page_offset = va & 0xFFF;
        let vpn2 = (va >> 30) & 0b111_111_111;
        uart.debug_value("vpn2", vpn2 as u64);
        let vpn1 = (va >> 21) & 0b111_111_111;
        uart.debug_value("vpn1", vpn1 as u64);
        let vpn0 = (va >> 12) & 0b111_111_111;
        uart.debug_value("vpn0", vpn0 as u64);

        let root_table = root_page as *mut [PTE39; 512];
        let entry2 = &(*root_table)[vpn2];
        uart.debug("entry2", entry2);
        let half_page = entry2.ppn() << 12;
        uart.debug_value("half_page", half_page);

        let half_table = half_page as *mut [PTE39; 512];
        let entry1 = &(*half_table)[vpn1];
        uart.debug("entry1", entry1);
        let leaf_page = entry1.ppn() << 12;
        uart.debug_value("leaf_page", leaf_page);

        let leaf_table = leaf_page as *mut [PTE39; 512];
        let entry0 = &(*leaf_table)[vpn0];
        uart.debug("entry0", entry0);
        let pa = (entry0.ppn() << 12) + page_offset as u64;
        uart.debug_value("pa", pa);
    }
}

fn ident_map_new_page(
    uart: &mut Uart,
    alloc: &BumpAllocator,
    mode: &PagingMode,
    root_page: usize,
    pa: usize,
) {
    uart.debug_value("Mapping address", pa as u64);

    unsafe {
        match mode {
            PagingMode::SV39 => {
                let root_table = root_page as *mut [PTE39; 512];

                let ppn2 = (pa >> 30) & 0b111_111_111;
                let ppn1 = (pa >> 21) & 0b111_111_111;
                let ppn0 = (pa >> 12) & 0b111_111_111;

                let x = &(*root_table)[ppn2];
                if !x.valid() {
                    // debug_value(uart, "ppn2", ppn2 as u64);
                    // let _ = uart.write_str("Invalid entry2\n");
                    let addr = alloc.allocate_new_page();
                    (*root_table)[ppn2] = PTE39::new().with_valid(true).with_ppn(addr as u64 >> 12);
                }
                let x = &(*root_table)[ppn2];

                let half_table = (x.ppn() << 12) as *mut [PTE39; 512];

                let y = &(*half_table)[ppn1];
                if !y.valid() {
                    // debug_value(uart, "ppn1", ppn1 as u64);
                    // let _ = uart.write_str("Invalid entry1\n");
                    let addr = alloc.allocate_new_page();
                    (*half_table)[ppn1] = PTE39::new().with_valid(true).with_ppn(addr as u64 >> 12);
                }
                let y = &(*half_table)[ppn1];

                let leaf_table = (y.ppn() << 12) as *mut [PTE39; 512];

                let z = &(*leaf_table)[ppn0];
                if !z.valid() {
                    // debug_value(uart, "ppn0", ppn0 as u64);
                    // let _ = uart.write_str("Invalid entry0\n");
                    (*leaf_table)[ppn0] = PTE39::new()
                        .with_readable(true)
                        .with_writable(true)
                        .with_executable(true)
                        .with_valid(true)
                        .with_global(true)
                        .with_accessed(true)
                        .with_dirty(true)
                        .with_ppn(pa as u64 >> 12);
                }
            }
        }
    }
}

fn enable_paging(uart: &mut Uart, _mode: &PagingMode, root_page: usize) {
    let satp = SatpRegister::new()
        .with_mode(8)
        .with_asid(0)
        .with_ppn(root_page as u64 >> 12);

    let satp_in: u64 = satp.into();

    let satp_out: u64;
    unsafe {
        asm!("
            CSRW satp, {0}
            sfence.vma zero, zero
            CSRR {1}, satp
        ", in(reg) satp_in, out(reg) satp_out);
    }

    uart.debug_value("satp", satp_out);
}
