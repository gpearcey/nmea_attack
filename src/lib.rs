//links to C++ functions
extern "C" {
    #[link_name = "printHello"]
    fn printHello(number: i32) -> i32;

    #[link_name = "SendMsgToApp"]
    fn SendMsgToApp() -> &'static str;

    #[link_name = "ReceiveMsgFromApp"]
    fn ReceiveMsgFromApp(controller_number: i32, PGN: i32, source: i32, priority: i32, data_length_bytes: i32, data: [char; 223] ) -> bool;

}



#[no_mangle]
fn main() {

    unsafe{
        let x: i32;
        x = printHello(11);
        printHello(x);
        let my_array: [char; 223] = ['x'; 223];
        //ReceiveMsgFromApp(0,12757,3,5,8,my_array);
        SendMsgToApp();

    }
}