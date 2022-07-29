pub fn parse(inp: String) -> Command {
	let mut outp = Command {
		action: String::new(),
		args: vec![]
	};

	let mut vec = vec![];

	let mut buf = String::new();

	let mut in_str = false;

	for c in inp.chars() {
		if c == '"' {
			in_str = !in_str;
			if !in_str {
				vec.push(buf.clone());
				buf = String::new();
			}
		}
		else if !in_str && c == ' ' {
			vec.push(buf.clone());
			buf = String::new();
		}
		else {
			buf.push((&c).to_owned());
		}
	}
	vec.push(buf.clone());

	if vec.len() > 0 {
		outp.action = vec[0].clone();
		outp.args = vec[1..].to_vec();
	}

	outp
}

pub struct Command {
	pub action: String,
	pub args: Vec<String>
}