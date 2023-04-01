mod jogadores;
use jogadores::Jogadores;

use std::io;
use crate::Leitura::{AdicionaJogador, RemoveJogador, TrocaJogador};

enum Leitura {
    RemoveJogador(usize),
    AdicionaJogador(usize),
    TrocaJogador(usize, usize),
    Quit,
    None
}

fn le_jogadores() -> Vec<String> {
    let mut jogadores = Vec::new();
    println!("Insira seu elenco aqui (até 11 jogadores)");
    for _ in 0..11 {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() || input.trim().is_empty() {
            break;
        } else {
            jogadores.push(input);
        }
    }
    jogadores
}

fn parse_jogadores(jogadores: &[String]) -> Vec<usize> {
    let mut new_jogadores = Vec::new();
    jogadores.iter().filter(|str| !str.is_empty()).for_each(|str| {
        let mut numeros = str.trim().split(' ');
        let rating = numeros
            .next()
            .expect("")
            .parse::<usize>()
            .expect("Primeiro argumento deveria ser um número");
        match numeros.next() {
            Some(vezes) => {
                let vezes = vezes.parse::<usize>().expect("Segundo argumento deveria ser um número");
                for _ in 0..vezes { new_jogadores.push(rating) }
            },
            None => { new_jogadores.push(rating) },
        };
    });
    new_jogadores
}

fn mostra_resultados(jogadores: &[usize], media_real: &f64, excesso: &f64, media: &f64) {
    print!("Seus {} jogadores: ", jogadores.len());
    for jogador in jogadores { print!("{jogador}, ") }
    println!("\nMédia real: {media_real:.3}");
    println!("Excesso: {excesso:.3}");
    println!("Média aritmética: {media:.3}\n");

    println!("Trocar jogador: 't [antigo] [novo]'");
    println!("Remover jogador: 'r [jogador]'");
    println!("Adicionar jogador: 'a [jogador]'");
    println!("Sair do programa: 'q'");
}

fn le_novo_input() -> Leitura {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Não foi possível ler sua entrada.");
    let mut line = input.trim().split(' ');

    if let Some(value) = line.next() {
        match value {
            "t" | "a" | "r" => {
                if let Ok(rating) = line.next().expect("Operação deve ter um parâmetro.").parse::<usize>() {
                    match value {
                        "a" => AdicionaJogador(rating),
                        "r" => RemoveJogador(rating),
                        "t" => {
                            let novo = line.next().expect("Operação deve ter dois parâmetros.").parse::<usize>().expect("Parâmetro deve ser um número");
                            TrocaJogador(rating, novo)
                        }
                        _ => { Leitura::None }
                    }
                } else {
                    Leitura::None
                }
            }
            "q" => { Leitura::Quit }
            _ => { Leitura::None }
        }
    } else {
        Leitura::None
    }
}

pub fn run() {
    let mut jogadores = Jogadores::new();
    {
        let jogadores_lidos = le_jogadores();
        let mut jogadores_lidos = parse_jogadores(&jogadores_lidos);
        // jogadores_lidos.iter().map(|&classificacao| jogadores.push(classificacao)).collect::<()>();
        jogadores_lidos.iter().for_each(|&classificacao| jogadores.push(classificacao));
    }

    // let mut total: f64;
    let mut media: f64;
    let mut excesso: f64;
    let mut media_real: f64;

    let mut quit = false;
    while !quit {
        // total = jogadores.total() as f64;
        media = jogadores.media_inicial();
        excesso = jogadores.excesso();
        media_real = jogadores.media_real();

        mostra_resultados(&jogadores.get_classificacoes(), &media_real, &excesso, &media);

        match le_novo_input() {
            Leitura::TrocaJogador(antigo, novo) => { jogadores.troca(antigo, novo); }
            Leitura::AdicionaJogador(novo) => { jogadores.push(novo); }
            Leitura::RemoveJogador(antigo) => { jogadores.remove(antigo); }
            Leitura::Quit => { quit = true; }
            Leitura::None => {}
        }
    }
}


#[cfg(test)]
mod tests {
    use float_cmp::{ApproxEq};
    use super::*;

    // #[test]
    // fn media() {
    //     let jogadores = vec![90, 89, 89, 88, 88, 87, 86, 84, 84, 84];
    //     let (soma, media) = media_jogadores(&jogadores, None);
    //     assert!( 869.0.approx_eq(soma, (0.0000001, 2)) );
    //     assert!( 86.9.approx_eq(media, (0.0000001, 2)) );
    //     let (soma, media) = media_jogadores(&jogadores, Some(10.5));
    //     assert!( 879.5.approx_eq(soma, (0.0000001, 2)) );
    //     assert!( 87.95.approx_eq(media, (0.0000001, 2)) );
    // }
    //
    // #[test]
    // fn parse() {
    //     let jogadores = vec!["89".to_string(), "90".to_string(), "87".to_string(), "84 3".to_string()];
    //     assert_eq!(vec![89, 90, 87, 84, 84, 84], parse_jogadores(&jogadores))
    // }
    //
    // #[test]
    // fn excesso() {
    //     let jogadores = vec![90, 89, 89, 88, 88, 87, 86, 84, 84, 84];
    //     let media = 86.9;
    //     assert!( 9.6.approx_eq(calcula_excesso(&jogadores, &media), (0.000001, 2)) );
    // }
    //
    // #[test]
    // fn troca() {
    //     let mut jogadores = vec![90, 89, 88, 88, 88, 87, 86, 85];
    //     troca_jogadores(&mut jogadores, 89, 88);
    //     assert_eq!(jogadores, [90, 88, 88, 88, 88, 87, 86, 85]);
    // }
}
