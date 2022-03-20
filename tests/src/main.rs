mod agregateur;

use crate::agregateur::{Resumable, Tweet::Tweet};

fn main() {
    let tweet = Tweet {
        nom_utilisateur: String::from("jean"),
        contenu: String::from("Bien sûr, les amis, comme vous le savez probablement déjà"),
        reponse: false,
        retweet: false,
    };

    println!("1 nouveau tweet: {}", tweet.resumer());
}