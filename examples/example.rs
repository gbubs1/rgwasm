extern "C" {
	fn print_char(c: u8);
}

fn println(data: &str) {
	for &c in data.as_bytes() {
		unsafe { print_char(c) }
	}

	unsafe { print_char(b'\n') }
}

fn main() {
    println("Hello, world!");
}
