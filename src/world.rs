
pub fn show() -> String {
    String::from( "World!" )
}

#[cfg(test)]
mod tests {
    use world;
    #[test]
    fn it_works() {
        assert_eq!( world::show(), String::from( "World!" ) )
    }
}
