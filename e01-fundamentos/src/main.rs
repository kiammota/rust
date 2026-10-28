use guid_create::GUID;
#[derive(Debug)]
struct Tupla(String, String);
fn main() {
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
    let registro = Tupla {
        0: GUID::rand().to_string(),
        1: String::from("Kiam"),
    };
    println!("registro: {:?}", registro)
    // arrays ->

    let meuarray: [u128; 512] = [100; 512];

}
