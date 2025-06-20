use proc_mem_rs::{Module, Process, Signature};

pub fn attach(name: &str, module: &str) -> (Process, Module) {
    let proc = Process::with_name(name)
        .unwrap_or_else(|_| panic!("Failed to attach to process: {}", name));
    let module = proc
        .module(module)
        .unwrap_or_else(|_| panic!("Failed to find module: {}", module));
    (proc, module)
}

pub fn find(module: &Module, pattern: &str) -> usize {
    // proc_mem_rs uses singe '?'
    let pattern = pattern.replace("??", "?");
    let sig = Signature {
        name: "sig".to_string(),
        pattern: pattern.to_string(),
        offsets: vec![],
        extra: 0isize,
        relative: true,
        rip_relative: false,
        rip_offset: 0,
    };
    let addr = module
        .find_signature(&sig)
        .unwrap_or_else(|_| panic!("Failed to find pattern: {}", pattern));
    addr
}

pub fn write<T: Default>(proc: &Process, addr: usize, value: T) -> bool {
    proc.write_mem(addr, value)
}

#[allow(dead_code)]
pub fn read<T: Default>(proc: &Process, addr: usize) -> T {
    proc.read_mem::<T>(addr)
        .unwrap_or_else(|_| panic!("Failed read memory at: {} ", addr))
}
