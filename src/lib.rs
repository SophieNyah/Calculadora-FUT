use std::io;

fn ler_jogadores() -> Vec<String> {
    let mut jogadores = Vec::new();
    println!("Insira seu elenco aqui (até 11 jogadores)");
    for _ in 0..11 {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() || input.is_empty() {
            break;
        } else {
            jogadores.push(input);
        }
    }
    jogadores
}

fn parse_jogadores(jogadores: &[String]) -> Vec<usize> {
    jogadores
        .iter()
        .filter(|str| str.trim().parse::<usize>().is_ok())
        .map(|str| str.trim().parse::<usize>().unwrap())
        .collect()
}

fn media_inicial_jogadores(jogadores: &[usize]) -> (usize, f64) {
    let soma = jogadores
        .iter()
        .copied()
        .reduce(|acc, rating| acc + rating)
        .expect("Iterador não deveria estar vazio");
    (soma, (soma as f64) / (jogadores.len() as f64))
}

fn calcula_excesso(jogadores: &[usize], media: &f64) -> f64 {
    jogadores
        .iter()
        .filter(|&&rating| (rating as f64) > *media)
        .fold(0.0, |acc, &rating| acc + ((rating as f64) - *media))
}

pub fn run() {
    let jogadores = ler_jogadores();
    let jogadores = parse_jogadores(&jogadores);
    let (total, media) = media_inicial_jogadores(&jogadores);
    let excesso = calcula_excesso(&jogadores,&media);

    let media_real = ((total as f64) + excesso) / (jogadores.len() as f64);

    println!("Média real: {media_real:.3}");
    println!("Excesso: {excesso:.3}");
    println!("Média aritmética: {media:.3}");
}


#[cfg(test)]
mod tests {
    use float_cmp::{ApproxEq};
    use super::*;

    #[test]
    fn media() {
        let jogadores = vec![90, 89, 89, 88, 88, 87, 86, 84, 84, 84];
        let (soma, media) = media_inicial_jogadores(&jogadores);
        assert_eq!(869, soma);
        assert!( 86.9.approx_eq(media, (0.0000001, 2)) );
    }

    #[test]
    fn parse() {
        let jogadores = vec!["89".to_string(), "90".to_string(), "87".to_string(), "84".to_string()];
        assert_eq!(vec![89, 90, 87, 84], parse_jogadores(&jogadores))
    }

    #[test]
    fn excesso() {
        let jogadores = vec![90, 89, 89, 88, 88, 87, 86, 84, 84, 84];
        let media = 86.9;
        assert!( 9.6.approx_eq(calcula_excesso(&jogadores, &media), (0.000001, 2)) );
    }
}
