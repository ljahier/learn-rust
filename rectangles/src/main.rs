#[derive(Debug)]
struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

impl Rectangle {
    fn aire(&self) -> u32 {
        self.largeur * self.hauteur
    }

    fn peut_contenir(&self, rect2: &Self) -> bool {
        return self.hauteur > rect2.hauteur && self.largeur > rect2.largeur;
    }
}

fn main() {
    let rect1 = Rectangle { largeur: 30, hauteur: 50 };
    let rect2 = Rectangle { largeur: 10, hauteur: 40 };
    let rect3 = Rectangle { largeur: 40, hauteur: 40 };

    println!(
        "L'aire du rectangle est de {} pixels carrés.",
        Rectangle::aire(&rect1)
    );

    println!(
        "Le rectangle 1 {} contenir le rectangle 2",
        if rect1.peut_contenir(&rect2) == true { "peut" } else { "ne peut pas" }
    );

    println!(
        "Le rectangle 1 {} contenir le rectangle 3",
        if rect1.peut_contenir(&rect3) == true { "peut" } else { "ne peut pas" }
    );
}