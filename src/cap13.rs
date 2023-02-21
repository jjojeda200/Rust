#[derive(Debug, PartialEq, Copy, Clone)]
enum Color {
    Red,
    Blue,
    Black,
}

struct Inventory {
    shirts: Vec<Color>,
}

impl Inventory
{
    fn giveaway(&self, user_preference: Option<Color>) -> Color
    {
        user_preference.unwrap_or_else(|| self.most_stocked())

    }

    fn most_stocked(&self) -> Color
    {
        let mut num_black = 0;
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                Color::Red => num_red += 1,
                Color::Blue => num_blue += 1,
                Color::Black => num_black += 1,
            }
        }
        if num_red > num_blue && num_red > num_black
        {
            Color::Red
        } else {
            if num_blue > num_red && num_blue > num_black 
            {
                Color::Blue
            } else {
                Color::Black
            }
        }

    }
}

#[allow(dead_code)]
pub fn sorteo() {
    let store = Inventory {
        shirts: vec![Color::Black, Color::Black, Color::Blue, Color::Red, Color::Black, Color::Red],
    };

    println!("Número de camisas {}",store.shirts.len());
    println!("Mayor color en inventario {:?}", store.most_stocked());

    let user_pref1 = Some(Color::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "El usuario con preferencia {:?} Obtiene {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway1 = store.giveaway(user_pref2);
    println!(
        "El usuario con preferencia {:?} Obtiene {:?}",
        user_pref2, giveaway1
    );
    
    let user_pref3 = Some(Color::Blue);
    let giveaway1 = store.giveaway(user_pref3);
    println!(
        "El usuario con preferencia {:?} Obtiene {:?}",
        user_pref3, giveaway1
    );
}

//*****************************************************************************