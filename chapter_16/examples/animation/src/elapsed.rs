// See: https://notes.iveselov.info/programming/time_it-a-case-study-in-rust-macros

#[macro_export]
macro_rules! elapsed {
    ($($tt:tt)+) => {
        let timer = std::time::Instant::now();
        $(
            $tt
        )+
        let time = timer.elapsed().as_micros() as f64 / 1_000_000.0;
        let func_string = stringify!($($tt)+)
            .lines()
            .map(|line| format!("{}{}", " -> ", line))
            .collect::<Vec<String>>()
            .join("\n");
        println!("Elapsed seconds: {:.6}\n{}\n", time, func_string);
    }
}


// #[macro_export]
// macro_rules! elapsed {
//     ( $($func: expr);+ $(;)?) => (
//         let time = std::time::Instant::now();
//         $($func;)+
//         let time = time.elapsed().as_micros() as f64 / 1_000_000.0;
//         let func_string = stringify!($($func)+)
//             .lines()
//             .map(|line| format!("{}{}", "-> ", line))
//             .collect::<Vec<String>>()
//             .join("\n");
//         println!("Elapsed seconds: {:.6}\n{}\n", time, func_string);
//     )
// }