fn main() {
    println!("Hello, world aaa!");
}

#[test]
fn hello_test(){
    println!("testing testing...")
}

#[test]
fn test_variable(){
    let name = "Kresna Wijaya";
    println!("{} < < < apanich", name);
}

#[test]
fn test_muable(){
    let mut name = "Kresna Wijaya";
    println!("{} < < < apanich", name);

    name = "Kresna Aja";
    println!("{} < < < berubah", name);
}

#[test]
fn shadowing(){
    let name = "sepuluh";
    println!("Hello {}", name);

    let name = 10;
    println!("Hello {}", name);
}

#[test]
fn number_conversion(){
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    // error: integer overflow
    let d: i64 = 1000000000;
    let e: i8 = d as i8;
    println!("{}", e);
}

#[test]
fn numeric_operator(){
    let a = 10;
    let b = 10;

    let c = a * b;
    let d = a / b;
    let e = a + b;
    let f = a - b;

    println!("{}, {}, {}, {}", c, d, e, f);

    // augmented assignments

    let mut x = 10;
    println!("{}", x);

    x += 10;
    println!("{}", x);

    x *= 10;
    println!("{}", x);
}

#[test]
fn boolean(){
    let a = true;
    println!("apakah kresna tamvan? {}!!!", a);
}

#[test]
fn comparison_operator(){
    let a = 1;
    let b = 2;

    let c = a > b;
    println!("{}", c);
}

#[test]
fn boolean_operator(){
    let a = 10;
    let b = 20;

    if a > 10 && b > 10 {
        println!("keduanya lebih dari 10");
    } else{
        println!("ada yang kurang dari 10");
    }
}

#[test]
fn char_type(){
    let a = 'A';
    let b = 'B';

    println!("ini {}, ini {}", a, b);
}

#[test]
fn tuple(){
    let data: (i32, f64, bool) = (10, 2.5, true);
    println!("{:?}", data);

    let mut datamut: (i32, f64, bool) = (10, 2.5, true);

    datamut.0 = 20;

    println!("{:?}", datamut);
    println!("{}", datamut.0);

    let (a, b, _) = datamut;
    println!("{} {}", a, b);
}

#[test]
fn unit_tuple(){
    let res: () = main();
    println!("{:?}", res);
}

#[test]
fn array(){
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", array);

    let a = array[0];
    let b = array[1];

    println!("{} {}", a, b);

    let mut arraymut: [i32; 5] = [1, 2, 3, 4, 5];
    
    println!("{:?}", arraymut);

    arraymut[0] = 100;
    arraymut[1] = 200;

    println!("{:?}", arraymut);
    println!("{} {}", arraymut[0], arraymut[1]);

    let length = arraymut.len();
    println!("{} ini length array", length)
}

#[test]
fn two_dimensional_array(){
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6]
    ];

    println!("{:?}", matrix);

    let mut matrixmut = [
        [1, 2],
        [3, 4],
    ];

    println!("{:?}", matrixmut);
    println!("{}", matrixmut[1][1]); // harusnya 'c'

    matrixmut[1][1] = 999;
    println!("{:?}", matrixmut);
    println!("{}", matrixmut[1][1]); // harusnya 'c'
}

#[test]
fn constant(){
    const MINIMUM: i32 = 0;
    const MAXIMUM: i32 = 100;

    println!("{} {}", MINIMUM, MAXIMUM);
}

#[test]
fn variable_scope(){
    let str1 = "satu";
    {
        let str2 = "dua";
        println!("{} {}", str1, str2);
    }
    //println!("{} {}", str1, str2); // eror: str2 di dalem scope
}

#[test]
fn stack_heap(){
    function_a();
    function_b();
}

fn function_a(){
    let a = 10;
    let b = String::from("Kresna");

    println!("{} {}", a, b);
}

fn function_b(){
    let a = 10;
    let b = String::from("Wijaya");

    println!("{} {}", a, b);
}

#[test]
fn string_slices(){
    let name = "   Kresna Wijaya   ";
    let trim = name.trim();

    println!("{}", name);
    println!("{}", trim);
    println!("{}", name);
}

#[test]
fn string_type(){
    let mut name = String::from("Kresna Wijaya");
    println!("{}", name);

    name.push_str(" Tamvan");
    println!("{}", name);

    let messi = name.replace("Kresna", "Messi");
    println!("{}", messi);
}

#[test]
fn ownership_rules() {
    // disini gak bisa akses a, belum deklarasi
    let a = 10; // mulai dari sini bisa akses a

    { // disini gak bisa akses b, belum deklarasi
        let b = 20; // mulai dari sini bisa akses b
        
        println!("{}", b);
    } // scope b selesai, b dihapus, b gak bisa diakses lagi

    println!("{}", a);
} // scope a selesai, a dihapus, a gak bisa diakses lagi

#[test]
fn data_copy() {
    let a = 10;
    let mut b = a;

    println!("{} {}", a, b);

    b = 20;
    println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
    let a = String::from("Kresna");
    println!("{}", a);

    let mut b = a; // ownership pindah ke b
    println!("{}", b);

    // println!("{} {}", a, b); // a bakal eror, karena udah pindah ke b

    b = String::from("Wijaya");
    println!("{}", b);
}

#[test]
fn clone() {
    let a = String::from("Kresna");
    let b = a.clone(); // ownership gak pindah, di duplicate

    println!("{} {}", a, b);
}

#[test]
fn if_expression() {
    let value = 20;

    let result = if value < 10 {
        "kuranggg..."
    } else if value > 15 {
        "kebanyakan..."
    } else {
        "mantap..."
    };
    
    println!("{}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;

    loop {
        counter += 1;

        if counter == 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("Counter: {}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result: {}", result);
}

#[test]
fn loop_labe() {
    let mut i = 1;
    'outer: loop {
        let mut j = 1;
        loop {
            if i > 3 {
                break 'outer;
            }

            let res = i * j;
            println!("{} x {} = {}", i, j, res);
            j += 1;

            if j > 3 {
                break;
            }
        }
        i += 1; 
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 1 {
            println!("Counter: {}", counter);
        }
        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < array.len() {
        println!("Array index ke-{}: {}", index, array[index]);
        index += 1;
    }

    println!("= = = = =");

    for value in array {
        println!("Value: {}", value);
    }
}

#[test]
fn range() {   
    let array = [1, 2, 3, 4, 5];

    let range = 0..5;
    println!("{}", range.start);
    println!("{}", range.end);

    for value in range {
        println!("Value: {}", array[value]);
    }
}

fn say_hello(){
    println!("Hello!")
}

#[test]
fn test_say_hello() {
    say_hello();
    say_hello();
}

fn say_goodbye(first_name: &str, last_name: &str){
    println!("Goodbye {} {}!", first_name, last_name);
}

#[test]
fn test_say_goodbye() {
    say_goodbye("Lionel", "Messi");
    say_goodbye("Christiano", "Ronaldo");
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
        println!("ini loop: {}", result);
    }

    println!("= = = = =");
    result
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(5);

    println!("{}", result);
}

fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    print_text(value, times - 1);
}

#[test]
fn test_print_text() {
    print_text(String::from("Hello"), 5);
}

fn factorial_recursive(n: u32) -> u32 {
    if n < 1 {
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("{}", result);
}