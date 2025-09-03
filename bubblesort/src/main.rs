
fn populate_array(list: &mut [i32; 10]) -> () {
}

fn print_array(list: &mut [i32; 10]) -> () {
    print!("[");
    for i in 0..(list.len() - 1){
        print!("{}, ", i);
    }
    println!("{}]", list[list.len() - 1]);
}


fn main() {
    let mut l: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    print_array(&mut l);
}

