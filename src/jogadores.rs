use rs_mois_dirty::Dirty;

pub struct Jogadores {
    classificacoes: Vec<usize>,
    soma: usize,
    media_inicial: Dirty<f64>,
    excesso: Dirty<f64>,
    media_real: Dirty<f64>,
}

impl Jogadores {
    pub fn new() -> Self {
        Self {
            classificacoes: Vec::new(),
            soma: 0,
            media_inicial: Default::default(),
            excesso: Default::default(),
            media_real: Default::default(),
        }
    }

    pub fn get_classificacoes(&self) -> &[usize] {
        self.classificacoes.as_slice()
    }

    fn dirty_all(&mut self) {
        self.media_inicial.set_dirty();
        self.excesso.set_dirty();
        self.media_real.set_dirty();
    }

    pub fn push(&mut self, rating: usize) {
        if self.classificacoes.len() >= 11 { return }
        self.classificacoes.push(rating);
        self.soma += rating;
        self.dirty_all();
        self.classificacoes.sort_unstable_by(|a, b| b.cmp(a));
    }

    pub fn media_inicial(&mut self) -> f64 {
        *self.media_inicial.update_if_dirty(|| (self.soma as f64) / (self.classificacoes.len() as f64))
    }

    pub fn excesso(&mut self) -> f64 {
        let media = self.media_inicial();
        *self.excesso.update_if_dirty(
            || self.classificacoes
                .iter()
                .filter(|&&rating| (rating as f64) > media)
                .fold(0.0, |acc, &rating| acc + ((rating as f64) - media))
        )
    }

    pub fn media_real(&mut self) -> f64 {
        let excesso = self.excesso();
        *self.media_real.update_if_dirty(|| ((self.soma as f64) + excesso) / (self.classificacoes.len() as f64))
    }

    pub fn remove(&mut self, item: usize) -> bool {
        if let Some(posicao) = self.classificacoes.iter().position(|x| *x == item) {
            self.soma -= item;
            self.dirty_all();
            self.classificacoes.remove(posicao);
            true
        } else {
            false
        }
    }

    pub fn troca(&mut self, antigo: usize, novo: usize) -> bool {
        match self.remove(antigo) {
            true => {
                self.push(novo);
                true
            },
            _ => { false },
        }
    }

    pub fn total(&self) -> usize {
        self.soma
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::{ApproxEq};
    use super::*;

    #[test]
    fn new() {
        let jogadores = Jogadores::new();
        assert_eq!(jogadores.classificacoes, []);
        assert_eq!(jogadores.soma, 0);
    }

    #[test]
    fn push() {
        let mut jogadores = Jogadores::new();
        jogadores.push(89);
        jogadores.push(90);
        jogadores.push(89);
        assert_eq!(jogadores.classificacoes, [90, 89, 89]);
    }

    #[test]
    fn media_inicial() {
        let mut jogadores = Jogadores::new();
        jogadores.push(90);
        jogadores.push(89);
        jogadores.push(89);
        jogadores.push(88);
        assert!( 89.0.approx_eq(jogadores.media_inicial(), (0.00000001, 2)) );
    }

    #[test]
    fn excesso() {
        let mut jogadores = Jogadores::new();
        jogadores.push(90);
        jogadores.push(89);
        jogadores.push(89);
        jogadores.push(88);
        assert!( 1.0.approx_eq(jogadores.excesso(), (0.00000001, 2)) );
    }

    #[test]
    fn media_real() {
        let mut jogadores = Jogadores::new();
        jogadores.push(90);
        jogadores.push(89);
        jogadores.push(89);
        jogadores.push(88);
        assert!( 89.25.approx_eq(jogadores.media_real(), (0.00000001, 2)) );
    }

    #[test]
    fn total() {
        let mut jogadores = Jogadores::new();
        jogadores.push(90);
        jogadores.push(89);
        jogadores.push(89);
        jogadores.push(88);
        assert_eq!(jogadores.total(), 356);
    }

    #[test]
    fn remove() {
        let mut jogadores = Jogadores::new();
        jogadores.push(90);
        jogadores.push(89);
        jogadores.push(89);
        jogadores.push(88);
        jogadores.remove(88);
        jogadores.remove(70);
        assert_eq!(jogadores.classificacoes, [90, 89, 89]);
    }

    #[test]
    fn troca() {
        let mut jogadores = Jogadores::new();
        jogadores.push(90);
        jogadores.push(89);
        jogadores.push(89);
        jogadores.push(88);
        jogadores.troca(89, 87);
        jogadores.troca(80, 90);
        assert_eq!(jogadores.classificacoes, [90, 89, 88, 87]);
    }

}