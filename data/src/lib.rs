
#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

pub fn get_dataset() -> Point  {
    let point = Point { x: 1, y: 2 };
    
    point
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
