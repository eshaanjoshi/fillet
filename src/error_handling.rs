
static mut HAS_ERROR:bool = false;
pub fn error(line:i32, message:String )
{
    report(line, "".to_string(), message);
}



fn report(line:i32, location:String, message:String)
{
    eprintln!("Line {line_no} Error {whe} : {mes}", line_no=line, whe=location, mes=message);
    
    unsafe { HAS_ERROR=true };
}