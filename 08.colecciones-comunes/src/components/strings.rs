pub mod string {
    

    
    pub fn string() {
        let s = String::from("initial contents \n");
        print!("{}", s);
    }

    pub fn actualizando_strings(){
        let mut s = String::from("lo");
        s.push('l');
        print!("{}", s);

    }

    pub fn concat(){
        let s1 = String::from("Hey bro ");
        let s2 = String::from("what's up? \n");
        // let s3 = s1 + &s2;
        // print!("{}", s3);
        let s = format!("{s1}{s2}");
        print!("{}", s);
    }

    pub fn indexacion(){
        let s1: String = String::from("value");
        let h: &str = &s1[0..1];
        print!("{}", h);

        for c in s1.chars(){
            print!("{}", c);
        }

        for b in s1.bytes(){
            print!("{}\n", b);
        }
    }
}
