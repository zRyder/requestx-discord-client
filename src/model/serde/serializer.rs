pub struct RequestSerializer<W> {
	writer: W
}

impl<W> RequestSerializer<W> {
	pub fn new(writer: W) -> Self { RequestSerializer { writer } }
}
