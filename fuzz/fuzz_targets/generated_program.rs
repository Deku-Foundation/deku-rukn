#![no_main]

use libfuzzer_sys::fuzz_target;
use rukn::run;

fn byte_at(input: &[u8], index: usize) -> u8 {
    input.get(index).copied().unwrap_or(0)
}

fn arabic_digit(value: u8) -> char {
    char::from_u32(0x0660 + u32::from(value % 10)).expect("Arabic-Indic digit is valid")
}

fn number(input: &[u8], index: usize) -> char {
    arabic_digit(byte_at(input, index))
}

fn generated_program(input: &[u8]) -> String {
    let a = number(input, 1);
    let b = number(input, 2);
    let c = number(input, 3);
    let n = byte_at(input, 4) % 8;
    match byte_at(input, 0) % 8 {
        0 => {
            let operator = ["+", "-", "*", "<", "=="][usize::from(byte_at(input, 5)) % 5];
            format!("دع ألف = {a}؛ دع باء = {b}؛ اطبع(ألف {operator} باء)؛")
        }
        1 => format!(
            "دع قائمة = [{a}, {b}]؛ دع جديدة = أضف(قائمة, {c})؛ اطبع(قائمة[٠])؛ اطبع(طول(جديدة))؛"
        ),
        2 => format!(
            "دالة جمع(س, ص) {{ أرجع س + ص؛ }} دالة طبق(عملية, س, ص) {{ أرجع عملية(س, ص)؛ }} اطبع(طبق(جمع, {a}, {b}))؛"
        ),
        3 => {
            let boolean = if byte_at(input, 5) % 2 == 0 {
                "صحيح"
            } else {
                "خطأ"
            };
            format!("إذا ({boolean}) {{ اطبع(«نعم»)؛ }} وإلا {{ اطبع(«لا»)؛ }}")
        }
        4 => format!("دع عد = ٠؛ بينما (عد < {n}) {{ عد = عد + ١؛ }} اطبع(عد)؛"),
        5 => format!(
            "اطبع(«أهلًا»[{}])؛ اطبع(طول(«أهلًا»))؛",
            arabic_digit(byte_at(input, 5) % 5)
        ),
        6 => "دع أ = ٧؛ اطبع(أ)؛".to_owned(),
        _ => {
            let mut source = format!("دع قيمة = {a}؛");
            for _ in 0..(byte_at(input, 5) % 12) {
                source.push_str(" قيمة = أضف([], قيمة)؛");
            }
            source.push_str(" اطبع(طول([]))؛");
            source
        }
    }
}

fuzz_target!(|input: &[u8]| {
    // Generate bounded, mostly valid Rukn programs from language-feature templates.
    // The separate run_source target continues to fuzz arbitrary UTF-8-decoded bytes.
    let source = generated_program(input);
    let _ = run(&source);
});
