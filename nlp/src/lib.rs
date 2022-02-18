use data;

pub fn train(p: &data::Point) {
    println!("Input data: {:?}",p);
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
