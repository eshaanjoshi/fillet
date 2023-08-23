use std::process::exit;


static mut HAS_ERROR: bool = false;
///Barebones error handler, reports and error based on the specific problem caught.
pub fn error(line: usize, message: String) {
    report(line, "".to_string(), message);
}




pub fn type_error() {
    eprintln!("Typechecker Error, See output");
    //exit(1);
}

pub fn fatal_error(errortype:String, message:String, location:usize){
    eprintln!("FATAL {} ERROR: Error {} at line {}", errortype, message, location);
    exit(1);
}

///Underlying report function, private, used to report the location to cli
fn report(line: usize, location: String, message: String) {
    eprintln!(
        "Line {line_no} Error {whe} : {mes}",
        line_no = line,
        whe = location,
        mes = message
    );

    unsafe { HAS_ERROR = true };
}
