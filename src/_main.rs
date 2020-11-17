// #![feature(asm)]

// use std::env;

// fn main() {
//     let mut input = env::args()
//         .nth(1)
//         .expect("You must enter argument")
//         .chars()
//         .rev()
//         .collect::<String>();
//     input += "\0";

//     let input = &input[..];

//     let mut output: usize;

//     unsafe {
//         asm!("
//                 MOVZX ebx, BYTE PTR [{s}]
//                 MOV rdx, 1
//                 XOR rcx, rcx
//                 TEST bl, bl
//                 JE END
//             LOOP:
//                 ADD {s}, 1
//                 SUB rbx, 48
//                 IMUL rbx, rdx
//                 ADD rcx, rbx
//                 MOVZX ebx, BYTE PTR [{s}]
//                 IMUL rdx, 10
//                 TEST bl, bl
//                 JNE LOOP
//             END:
//                 MOV {o}, rcx
//             ",
//             s = in(reg) input as *const _ as *const u8,
//             o = out(reg) output,
//         )
//     }

//     println!("{}", output);
// }
