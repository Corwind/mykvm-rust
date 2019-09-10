extern crate kvm;
extern crate memmap;
extern crate libc;


use kvm::{Capability, Exit, IoDirection, System, Vcpu, VirtualMachine};
use memmap::{Mmap, Protection};
use libc::{E2BIG, ENOMEM, c_int};

fn main() {
    let mut anon_mmap = Mmap::anonymous(3 << 29, Protection::ReadWrite)
        .unwrap();

    let slice = unsafe { anon_mmap.as_mut_slice() };

    let sys = System::initialize().unwrap();

    let mut vm = VirtualMachine::create(&sys).unwrap();

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
