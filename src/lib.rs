//links to C++ functions
extern "C" {
    #[link_name = "printHello"]
    fn printHello(number: i32) -> i32;

    #[link_name = "printStr"]
    fn printStr(input: *const char) -> i32;

    #[link_name = "SendMsgToApp"]
    fn SendMsgToApp(char_ptr: *const char, length: i32) -> i32;

    #[link_name = "ReceiveMsgFromApp"]
    fn ReceiveMsgFromApp(controller_number: i32, PGN: i32, source: i32, priority: i32, data_length_bytes: i32, data: [char; 223] ) -> bool;

}

static mut GLOBAL_CHAR_PTR: *const char = std::ptr::null();

#[no_mangle]
pub extern "C" fn get_msg(char_arr: *const char, char_array_size: i32 ){
    unsafe {
        GLOBAL_CHAR_PTR = char_arr;
    }
}



#[no_mangle]
fn main() {

    unsafe{
        let x: i32;
        x = printHello(11);
        printHello(x);
        let first_char = unsafe {*GLOBAL_CHAR_PTR};
        
        printStr(GLOBAL_CHAR_PTR);

        //let my_array: [char; 223] = ['x'; 223];
        //ReceiveMsgFromApp(0,12757,3,5,8,my_array);
        //SendMsgToApp();

    }
}