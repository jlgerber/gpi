use gpi::GPI;
use std::env;

fn main() -> Result<(), gpi::GpiError> {
    let mut args = env::args();

    if let Some(package) = args.nth(1) {
        let pkg = GPI::get_record(&package)?;
        println!("{:#?}", pkg);
    } else {
        println!("please supply a package name");
    };
    Ok(())

}
