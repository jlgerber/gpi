use gpi::GPI;
use std::env;
use gpi::table_record;

fn main() -> Result<(), gpi::GpiError> {
    let mut args = env::args();

    if let Some(package) = args.nth(1) {
        let pkg = GPI::get_record(&package)?;
        table_record(&package, &pkg);
        //println!("{:#?}", pkg);
    } else {
        println!("please supply a package name");
    };
    Ok(())

}
