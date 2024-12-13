
   pub fn games() {
        let dice_roll: i32 = 5;
        match dice_roll {
            3 | 5 => add_fancy_hat(), // El valor que se llega a poner a la condicion es donde se va a ejecutar la funcion
            7 | 2 => remove_fancy_hat(),
            _ => (),
        }

        fn add_fancy_hat() {
            println!("Adding fancy hat!");
        }
        fn remove_fancy_hat() {
            println!("Removing fancy hat!");
        }
        // fn reroll(){
        //     println!("Rerolling!");
        // }
    }


