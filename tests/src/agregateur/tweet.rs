use crate::agregateur::Resumable;

pub struct Tweet {
    pub nom_utilisateur: String,
    pub contenu: String,
    pub reponse: bool,
    pub retweet: bool,
}

impl Resumable for Tweet {
    fn resumer(&self) -> String {
        format!("{} : {}", self.nom_utilisateur, self.contenu)
    }
}