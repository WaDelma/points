#![feature(plugin, custom_attribute)]
#![plugin(points)]

#[test]
#[points = "1"]
fn first_fn() {

}

#[points = "2"]
mod first {
    #[test]
    #[points = "2.1"]
    fn second_fn() {
    
    }
    #[test]
    #[points = "2.2"]
    fn third_fn() {
    
    }
}

#[test]
#[points = "3"]
fn fourth_fn() {

}
