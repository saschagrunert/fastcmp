use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("compare.rs");
    let mut f = File::create(&dest_path).unwrap();

    // Generate the slice comparison source code
    f.write_all(b"macro_rules! slice_compare (").unwrap();
    f.write_all(b"($a:expr, $b:expr, $c:expr, $d:expr, $len:expr) => {{\n").unwrap();
    f.write_all(b"match $len {\n").unwrap();

    for i in 1..257 {
        let mut bits = i * 8 as usize;
        let mut sizes = vec![8, 16, 32, 64];

        #[cfg(feature = "simd_support")]
        sizes.push(128);

        let mut offset = 0;

        write!(f, "{} => ", i).unwrap();
        while !sizes.is_empty() {
            let size = sizes.last().unwrap().clone();
            if bits >= size {
                if offset > 0 {
                    write!(f, " && ").unwrap();
                }
                match size {
                    128 => write!(f, "cmp_u128!($c, $d, $len, {})", offset).unwrap(),
                    _ => write!(f, "cmp!($a, $b, u{}, {})", size, offset).unwrap(),
                }
                bits = bits.checked_sub(size).unwrap();
                offset += size / 8;
            } else {
                sizes.pop();
            }
            if bits == 0 {
                break;
            }
        }
        write!(f, ",\n").unwrap();
    }

    f.write_all(b"_ => unsafe { memcmp($a, $b, $len) == 0 },").unwrap();
    f.write_all(b"}}});").unwrap();
}
