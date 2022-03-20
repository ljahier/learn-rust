pub mod Tweet;
#[path="article_de_presse.rs"]
pub mod ArticleDePresse;

pub trait Resumable {
    fn resumer(&self) -> String {
        String::from("(En savoir plus ...)")
    }
}