use regex::Regex;

/// ua a  a  funciton using rejek *** bruh *** omg guys `regkjeK???`
/// haha text
/// algo que uso el ***REGEX*** aaoiaauauauaauauauaaaauuuuuuuuuu
pub fn suck(inp:&String) -> bool {
	let re = Regex::new(r"(?i)^i have \d+ eggg*s$").unwrap();
	let cap = re.captures(&inp);
	println!("{:?}", cap);
	
	re.is_match(&inp)
}