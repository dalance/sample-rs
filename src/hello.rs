
pub fn show() -> String {
    String::from( "Hello" )
}

#[cfg(test)]
mod tests {
    use hello;
    #[test]
    fn it_works() {
        assert_eq!( hello::show(), String::from( "Hello" ) )
    }
}
