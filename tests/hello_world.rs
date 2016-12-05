extern crate sample;

#[cfg(test)]
mod tests {
    use sample::hello;
    use sample::world;
    #[test]
    fn it_works() {
        assert_eq!( format!( "{} {}", hello::show(), world::show() ), String::from( "Hello World!" ) )
    }
}
