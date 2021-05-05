use io_ops::readron::options;
use lazy_static::lazy_static;

// This is to initialize the options once and for all.
lazy_static! {
	pub static ref OPTIONS: options::Stuff = options::get();
}
