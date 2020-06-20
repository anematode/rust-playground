mod lib;
mod aaaaastrings;
mod regex;

fn main() {
	println!("exclamatory");
	let eggs:String = lib::eggman(5);
	println!("{}", &eggs);
	aaaaastrings::stringsrntfunsmhmh();
	aaaaastrings::thsisnotasrtring();

	let outegg:bool = regex::suck(&eggs);
	println!("{:?}", (outegg, regex::suck(&String::from("obama")), regex::suck(&String::from("i have 239592385293 egggggs"))));
}