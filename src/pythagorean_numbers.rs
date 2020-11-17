use std::{fs::File, io::Write};

#[repr(align(16))]
struct f32x4([f32; 4]);

pub fn run() {
    let mut file = File::create("results.csv").unwrap();
    file.write(b"X,Y,Z\n").unwrap();

    let mut pythagorean_numbers = vec![];

    for i in 1..=996 {
        let t1 = f32x4([i as f32; 4]);

        for j in (i..=996).step_by(4) {
            let t2 = f32x4([
                (j + 1) as f32,
                (j + 2) as f32,
                (j + 3) as f32,
                (j + 4) as f32,
            ]);

            let mut output = f32x4([0.; 4]);

            let mut z_values = f32x4([0.; 4]);

            unsafe {
                asm!(
                    "
                    MOVAPS xmm0, [{0}]
                    MOVAPS xmm1, [{1}]
                    
                    MOVAPS xmm2, xmm0
                    MULPS xmm2, xmm2

                    MOVAPS xmm3, xmm1
                    MULPS xmm3, xmm3

                    MOVAPS xmm4, xmm2
                    ADDPS xmm4, xmm3

                    MOVAPS xmm5, xmm4
                    SQRTPS xmm5, xmm5
                    ROUNDPS xmm5, xmm5, 1
                    MOVAPS [{2}], xmm5
                    MULPS xmm5, xmm5
                    SUBPS xmm5, xmm4 

                    MOVAPS [{3}], xmm5
                    ",
                    in(reg) &t1,
                    in(reg) &t2,
                    in(reg) &mut z_values,
                    in(reg) &mut output,
                    out("xmm0") _, out("xmm1") _, out("xmm2") _, out("xmm3") _,
                    out("xmm4") _, out("xmm5") _
                )
            }
            for l in 0..4 {
                // FILTER
                if output.0[l] == 0. {
                    pythagorean_numbers.push([t1.0[l], t2.0[l]]);
                    file.write(format!("{},{},{}\n", t1.0[l], t2.0[l], z_values.0[l]).as_bytes())
                        .unwrap();
                }
            }
        }
    }

    println!("Pythagorean numbers calculated and outputed to results.txt");
}
