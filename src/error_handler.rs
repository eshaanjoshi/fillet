
static mut HAS_ERROR: bool = false;
///Barebones error handler, reports and error based on the specific problem caught.
pub fn error(line: usize, message: String) {
    report(line, "".to_string(), message);
}


pub fn fatal_error() {
    eprintln!("Fatal Error, See output");
    //exit(1);
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
