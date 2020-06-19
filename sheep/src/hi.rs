mod thirsty;

/// Una función extraña.
pub fn access_me() {
    println!("[access_me] I have been TOUCHED. I touch no_access now");
    no_access();
    println!("weeee {}", thirsty::gutchen())
}

/// Una función ***PRIVADA***. Esto significa que no lo puedes usar. Si lo puedes usar... no quieres darse cuenta de lo que ocurre si lo usas. :)
fn no_access() {
    println!("[no_access] ok.")
}
