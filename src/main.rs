mod new_folder;
use new_folder::NewFolder;
use std::io;
use std::process::Command;

fn main() {
    // std::process::Command::new("clear").status().unwrap();
    let mut new = NewFolder::new();
    println!("====================== BEM-VINDO AO DIR BUILDER =======================");
    println!("INFORME O NOME DO PROJETO");
    new.set_name_project(input());

    println!("\n--------------------------------- MÓDULOS ---------------------------------");
    println!("INFORME A QUANTIDADE DE MÓDULOS");
    match input().parse() {
        Ok(n) => new.set_quantity_modules(n),
        Err(_) => println!("VALOR PADRÃO DEFINIDO '0' "),
    };
    loop {
        println!("\n-------------------------------- DIREÓRIOS --------------------------------");
        println!(
            "ADICIONAR NOVA(S) PASTA(S) (A), DELETAR PASTA(S) PADRÃO (D) OU DEIXE VAZIO PARA SAIR"
        );

        match &*input() {
            "A" | "C" | "ADD" | "CRIAR" | "ADICIONAR" => {
                //println!("LISTA DAS PASTAS PADRÕES");
                new.show_dirs();
                println!("ADD: INFORME O NOME DA(S) NOVA(S) PASTAS SEPARADAS POR VIRGULAS (,)");
                new.add_new_folders(input());
                new.show_dirs();
                continue;
            }
            "D" | "R" | "E" | "EXCLUIR" | "DELETAR" | "REMOVER" => {
                //println!("LISTA DAS PASTAS PADRÕES");
                new.show_dirs();
                println!(
                    "DEL: INFORME O NOME DA(S) PASTAS A SER DELETADAS SEPARADAS POR VIRGULA (,)"
                );
                new.remove_folders(input());
                new.show_dirs();
                continue;
            }
            "" => {
                new.create_dir_and_files();
                println!("# PROJETO CONSTRUÍDO #");
                break;
            }
            _ => {
                println!("OPÇÃO INVÁLIDA");
                continue;
            }
        };
    }
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}
fn input() -> String {
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("ERRO AO LER A ENTRADA!");
    str.trim().to_uppercase()
}
