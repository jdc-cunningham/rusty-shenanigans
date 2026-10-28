fn main() {
    // type annotation
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32; // 5, suffix 32 bit

    // default type
    let default_float = 3.0;
    let default_integer = 7;

    // infer type
    let mut inferred_type = 12;
    inferred_type = 4294967296i64; // inferred from here

    let mut mutable = 12;
    mutable = 21;

    // type can't be changed

    // overwrite with shadowing
    // does not delete previous variable but old variable can't be reached
    let mutable = true;

    // array signature Type T and length [T; length]
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    let my_tuple = (5u32, 1u8, true, -5.04f32);
}
