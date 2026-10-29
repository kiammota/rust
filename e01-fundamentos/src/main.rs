use guid_create::GUID;

type Guid = GUID;
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    let tupla: (String, String) = (Guid::rand().to_string(), String::from("Kiam"));
    // tuplas --
    //  trabalhando com variáveis em rust
    // let -> indica variavel
    // let <mut> <nome>: <tipo> = <rvalue>
    let nome: String = String::from("Kiam");
    let idade: u8 = 17;
    println!(
        "olá, meu nome é {name} e tenho {age} anos.",
        name = nome,
        age = idade
    );
    // criando registro de tuplas
    println!("registro: {:?}", tupla);
    // arrays ->

    let meuarray: [u128; 512] = [67; 512];
    // loop for:
    // diferentemente do C, que tem três operadores: a declaração, a condição e o que fazer enquanto
    // não satisfaz
    // em rust temos: i -> variável que recebe cada elemento do iterador
    // in -> cadeia que será percorrida
    // <num>..<até o outro> -> cadeia que será percorrida
    for i in 0..meuarray.len() {
        println!("valor: {}", meuarray[i]);
    }
    println!("cores: {}", Color::Red as i32);
    let mut professor_permissoes = 0x00;
}
