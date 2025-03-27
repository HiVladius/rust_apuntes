// use iter_closure::closure::closure;

//los iter son lazy lo que significa que no tienen efecto hasta que llamas a un metodo
//que consume el iterador

#[allow(unused, dead_code)]

pub fn iterator() {
    // closure();

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    //Trait iterator y el metodo next

    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }



    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
    
}


mod tests {
    #[test]
    pub fn iterator_demostration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        println!("Iterator demostration passed");
    }
    
}   


pub mod tests2 {
    
    pub fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);

        println!("Iterator sum passed");
    }
}


#[derive(Debug, PartialEq)]
pub struct Shoe{
    size: u32,
    style: String,
}

pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe>{
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}


#[cfg(test)]

mod test{
    use std::vec;

    use super::*;

    #[test]
    fn filters_by_size(){
        let shoes = vec![
            Shoe{size: 10, style: String::from("sneaker")},
            Shoe{size: 13, style: String::from("sandal")},
            Shoe{size: 10, style: String::from("boot")},
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe{size: 10, style: String::from("sneaker")},
                Shoe{size: 10, style: String::from("boot")},
            ]
        )
    }
}