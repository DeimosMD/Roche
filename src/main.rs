use crate::compilation::generate_bc_from_src;

mod os_integration;
mod byte_codes;
mod compilation;
mod keywords;

fn main() {
    let src: Vec<u8> = String::from(
        "
            fn test()!
            {
                main();
            }

            fn main()!
            {
                test();
                main();
                test();
            }
        "
    ).into_bytes();
    let bc: Vec<u8> = generate_bc_from_src(src);
    println!("{:?}", bc);
}
