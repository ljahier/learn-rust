use crate::agregateur::Resumable;

pub struct ArticleDePresse {
    pub titre: String,
    pub lieu: String,
    pub auteur: String,
    pub contenu: String,
}

impl Resumable for ArticleDePresse {
    fn resumer(&self) -> String {
        format!("{}, par {} ({})", self.titre, self.auteur, self.lieu)
    }
}
