use std::io;

pub fn run() {
    println!("Enter number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.chars().rev().skip(2).collect::<String>();

    let length = input.len() as i32;

    if length < 1 {
        return println!("Min input length: 1");
    }
    if length > 10 {
        return println!("Max input length: 10");
    }

    let input = &input[..];

    let output: i32;

    unsafe {
        asm!("
            MOVZX ebx, BYTE PTR [{s}]
            MOV edx, 1
            MOV ecx, 0

            LOOP:
                ADD {s}, 1
                SUB ebx, 48
                IMUL ebx, edx
                ADD ecx, ebx
                MOVZX ebx, BYTE PTR [{s}]
                IMUL edx, 10

                DEC	eax
                JNZ	LOOP
            ",
            s = in(reg) input as *const _ as *const u8,
            out("ecx") output,
            in("eax") length,
        )
    }

    println!("{}", output);
}
