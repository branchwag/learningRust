use std::arch::asm;
//https://doc.rust-lang.org/reference/inline-assembly.html

fn main() {
    let msg = b"Hello, world!\n";

    unsafe {
        asm!(
            "syscall",
            in("rax") 1,
            in("rdi") 1,
            in("rsi") msg.as_ptr(),
            in("rdx") msg.len(),
            out("rcx") _,
            out("r11") _,
        );
    }
}
