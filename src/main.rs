use std::env;
mod mem;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <fps>", args[0]);
        std::process::exit(1);
    }
    let fps: f32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: '{}' is not a valid float", args[1]);
            std::process::exit(1);
        }
    };
    let invfps = 1.0 / fps;
    let (proc, module) = mem::attach("eldenring.exe", "eldenring.exe");
    let mut addr = mem::find(&module, "89 73 ?? C7 ?? ?? ?? ?? ?? ?? EB ?? 89 73");
    addr = module.base_address() + addr + 6;
    let success = mem::write(&proc, addr, invfps);

    if success {
        println!("Succesfully changed fps lock to {}", fps);
    }
}
