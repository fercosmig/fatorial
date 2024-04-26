use std::io;

fn convert_to_int(str_valor: & String) -> i64
{
    let int_valor: i64 = str_valor.trim().parse::<i64>().unwrap();
    int_valor
}

fn main()
{
    let mut str_valor: String = String::new();
    let mut int_valor: i64;
    let mut fatorial: i64;
    
    println!("Digite um valor: ");
    io::stdin().read_line(&mut str_valor).expect("str_valor error");
    int_valor = convert_to_int(&str_valor);

    fatorial = 1;
    while int_valor > 0
    {
        fatorial *= int_valor;
        int_valor -= 1;
    }
    println!("{}! = {}", str_valor, fatorial);
}
