pub fn stringsrntfunsmhmh(){
	let sussusuussusususususss:&str = "eeeeeuuuuuuuuuuuuuuuoobl";
	let mut lessssss:String = String::from(format!("{} LOL {}", sussusuussusususususss, "u u u u u iiis "));

	let longgggggg:usize = sussusuussusususususss.len();

	lessssss.push('e');
	lessssss.push_str("bad");

	println!("cap:{}, emp:{}, cont:{}", lessssss.capacity(), lessssss.is_empty(), lessssss.contains("bad"));

	lessssss = lessssss.replace("uu", "o");
	let longgggggger:usize = lessssss.len();

	println!("{:?}", (sussusuussusususususss, &lessssss, longgggggg, longgggggger));

	assert_eq!(41, longgggggger);

	println!("{:?}", (std::mem::size_of_val(&lessssss), lessssss.capacity()));
}