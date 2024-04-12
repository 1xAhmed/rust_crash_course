fn main() {

    //  Tuple, remember tuple has the max limit to 12, you can still use it but with limited functionality
    let info = (1, 3.3, 000, 999);
    let infor: (u8, f64, i32) = (1, 2.5, 35);

    let jets = info.0;
    let fuel = info.1;
    let ammo = infor.2;
    let (jet, fuels, ammos) = infor;


    // Array, by contrast, store multple values of same type. You can specify them litrally with [] and commas or with a value on how many you want seperated by semi-colon. NOte that type annotation for array always uses semi-colon form, even when you specify all the literal values in array. Array's are limited to size 32 above which they loses most of their functionality. Arrays live on the stack by default. SO you will usally use `vec` or slices of vectors, instead of arrays
    let buf = [1, 2, 3];
    let buff = [0; 3];  // [value; howmany]

    let bufff: [u8; 3] = [1, 2, 3];
}
