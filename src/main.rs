extern crate image;

mod utils;

use utils::ProgramOptions;
use std::process;
use std::env;





fn main() {
	let mut debug_receipt = String::new();
	let mut options = ProgramOptions::default();
	
	let     args     = env::args().collect::<Vec<String>>();
	let mut arg_iter = args.iter();
	loop {
		let arg = arg_iter.next();
		if arg == None { break; }
		
		let argstr = arg.unwrap();
		match argstr.as_str() {
			"-C" => {
				let arg = arg_iter.next();
				if let Some(argstr) = arg {
					options.background_color =
						utils::parse_color(argstr, options.channel_order)
						.expect("The backgroundcolor must be a valid color.");
				} else {
					eprintln!("A value must be provided to the backgroundcolor flag.");
					process::exit(0);
				}
			}
			"-O" => {
				let arg = arg_iter.next();
				if let Some(argstr) = arg {
					options.channel_order =
						argstr.parse().expect(
							format!("Channel name \"{argstr}\" is not supported.").as_str()
						);
				} else {
					eprintln!("A value must be passed to the channel order flag.");
					process::exit(0);
				}
			}
			"-R" => { options.remove_subvisible_pixels = true; }
			
			_ => {}
		}
	}
	
	println!("{:?}", options);
}










/* Calin Z. Baenen, 2023/01/20. */