use crate::Professores::Permissoes;

mod Professores {

    pub enum Permissoes {
        PassarAtividade = 1 << 0,
        CriarAtvidadade = 1 << 1,
    }

    #[derive(Debug)]
    pub struct Professor {
        pub nome: String,
        permissoes: u32,
    }

    impl Professor {
        pub fn new(nome: &str, permissao: Permissoes) -> Self {
            return Self {
                nome: nome.to_string(),
                permissoes: permissao as u32,
            };
        }
    }
}

fn main() {
    let luanex = Professores::Professor::new("Luanex", Permissoes::PassarAtividade);
    println!("professor criado: {:?}", luanex);
}
