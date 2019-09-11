extern crate kvm;
extern crate memmap;


use kvm::{Capability, Exit, IoDirection, System, Vcpu, VirtualMachine};
use memmap::{Mmap, Protection};

use std::mem;
use std::convert::TryInto;
use std::io;
use std::io::prelude::*;
use std::fs::File;


#[repr(C)]
#[derive(Clone, Debug)]
pub struct MyKvm {
    pm_addr: *const u64,
    setup_addr: *const u64,
    initrd_addr: *const u64,
    initrd_size: u64,
}

fn main() {
    let mut anon_mmap = Mmap::anonymous(3 << 29, Protection::ReadWrite)
        .unwrap();

    let slice = unsafe { anon_mmap.as_mut_slice() };

    let sys = System::initialize().unwrap();

    let mut vm = VirtualMachine::create(&sys).unwrap();

    let slice_ptr = slice.as_mut_ptr() as *mut u8;

    unsafe {

        let gdt_ptr = slice_ptr.offset(0x10000) as *mut u64;

        *gdt_ptr.offset(0) = 0;
        *gdt_ptr.offset(1) = 0x008f8b000000ffff;
        *gdt_ptr.offset(2) = 0x00af9b000000ffff;
        *gdt_ptr.offset(3) = 0x00cf93000000ffff;
    };

    assert!(vm.check_capability(Capability::UserMemory) > 0);
    vm.set_user_memory_region(0, slice, 0).unwrap();

    let mut vcpu = Vcpu::create(&mut vm).unwrap();

    let mut cpuid = sys.get_supported_cpuid().unwrap();
    vcpu.set_cpuid2(&mut cpuid).unwrap();

    let mut sregs = vcpu.get_sregs().unwrap();

    set_segment_selector(&mut sregs.cs, 0, !0, 1, 1, 0, 0, 0, 1, 11);
    set_segment_selector(&mut sregs.ds, 0, !0, 1, 1, 0, 0, 0, 1, 3);
    set_segment_selector(&mut sregs.es, 0, !0, 1, 1, 0, 0, 0, 1, 3);
    set_segment_selector(&mut sregs.fs, 0, !0, 1, 1, 0, 0, 0, 1, 3);
    set_segment_selector(&mut sregs.ss, 0, !0, 1, 1, 0, 0, 0, 1, 3);
    set_segment_selector(&mut sregs.tr, 0, !0, 0, 0, 0, 0, 0, 0, 11);

    sregs.cr0 |= 1;

    sregs.gdt.base = 0x10000;
    sregs.gdt.limit = (mem::size_of::<u64>() * 4 - 1).try_into().unwrap();

    sregs.idt.base = 0x11500;
    sregs.idt.limit = 7;

    vcpu.set_sregs(&sregs).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
}

fn set_segment_selector(seg : &mut kvm::Segment,
                        base: u64,
                        limit: u32,
                        g: u8,
                        db: u8,
                        l: u8,
                        avl: u8,
                        dpl: u8,
                        s: u8,
                        _type: u8) {
    seg.base = base;
    seg.limit = limit;
    seg.g = g;
    seg.db = db;
    seg.l = l;
    seg.avl = avl;
    seg.dpl = dpl;
    seg.s = s;
    seg._type = _type;
    seg.present = 1;
}

fn load_kernel(filename: String, mkvm: &MyKvm) {
    let file = File::open(&filename).unwrap();
}
