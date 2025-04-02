/// Determina la longitud de la secuencia de Collatz que empieza por `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut longitud: u32 = 0;

    loop {
        longitud += 1;
        //println!("n: {n}  longitud: {longitud}");

        if n == 1 {            
            break;
        } else if n % 2 == 0 {
            n = n / 2;                
        } else {
            n = 3 * n + 1;            
        }                
    }

    longitud
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}
  
 fn main() {
    println!("Longitud: {}", collatz_length(11));
 }