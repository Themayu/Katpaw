use image::{ImageFormat, ColorType};
use std::str::FromStr;





#[derive(PartialEq, Default, Clone, Debug, Copy, Eq)]
pub(crate) enum ChannelOrder {
	Argb,
	#[default] Rgba,
	Rgb
}

impl FromStr for ChannelOrder {
	type Err = ();
	fn from_str(s:&str) -> Result<Self, Self::Err> {
		match s {
			"argb" => Ok(Self::Argb),
			"rgba" => Ok(Self::Rgba),
			"rgb"  => Ok(Self::Rgb),
			_ => Err(())
		}
	}
}



#[derive(PartialEq, Default, Clone, Debug, Copy, Eq)]
pub(crate) struct ImageMargins {
	pub margin_bottom:usize,
	pub margin_right:usize,
	pub margin_left:usize,
	pub margin_top:usize
}



#[derive(PartialEq, Clone, Debug, Copy)]
pub(crate) struct ProgramOptions {
	/// Remove pixels that can't easily be distinguished from the background.
	pub remove_subvisible_pixels:bool,
	/// A formula that takes the background color and returns the actual color with
	/// transparency.  
	/// Having none means you can not interpret solid colors as transparent ones.
	pub transparency_decompiler:Option<fn(u32)->u32>,
	/// How transparent a pixel has to be before it's consdered subvisible.
	pub visibility_threshold:f32,
	/// The color that is considered "the background".
	pub background_color:u32,
	pub preserve_bottom:usize,
	pub preserve_right:usize,
	pub channel_order:ChannelOrder,
	pub preserve_left:usize,
	pub preserve_top:usize,
	/// The image format of the output.
	pub out_format:Option<ImageFormat>,
	/// The image format of the input.
	pub in_format:Option<ImageFormat>
}

impl Default for ProgramOptions {
	fn default() -> Self {
		Self {
			remove_subvisible_pixels:false,
			transparency_decompiler:None,
			visibility_threshold:0.03,
			background_color:0,
			preserve_bottom:0,
			preserve_right:0,
			channel_order:ChannelOrder::Rgba,
			preserve_left:0,
			preserve_top:0,
			out_format:None,
			in_format:None
		}
	}
}





pub(crate) fn imageformat_from_name(s:&str) -> Option<ImageFormat> {
	match s.to_lowercase().as_str() {
		"bitmap" | "bmp" => Some(ImageFormat::Bmp),
		"jpeg" | "jpg"   => Some(ImageFormat::Jpeg),
		"webp"           => Some(ImageFormat::WebP),
		"gif"            => Some(ImageFormat::Gif),
		"ico"            => Some(ImageFormat::Ico),
		"png"            => Some(ImageFormat::Png),
		"pnm"            => Some(ImageFormat::Pnm),
		
		_ => None
	}
}



pub(crate) fn colortype_from_name(s:&str) -> Option<ColorType> {
	match s.to_lowercase().as_str() {
		// Grayscale with Transparency
		"luminancealpha8" | "la8"   => Some(ColorType::La8),
		"luminancealpha16" | "la16" => Some(ColorType::La16),
		
		// Grayscale
		"luminance8" | "l8"   => Some(ColorType::L8),
		"luminance16" | "l16" => Some(ColorType::L16),
		
		// RGBA
		"rgba8"              => Some(ColorType::Rgba8),
		"rgba16"             => Some(ColorType::Rgba16),
		"rgba32" | "rgba32f" => Some(ColorType::Rgba32F),
		
		// RGB
		"rgb8"             => Some(ColorType::Rgb8),
		"rgb16"            => Some(ColorType::Rgb16),
		"rgb32" | "rgb32f" => Some(ColorType::Rgb32F),
		
		_ => None
	}
}



/// Parses a hexadecimal colorcode.
pub(crate) fn parse_color(cs:&str, co:ChannelOrder) -> Result<u32, ()> {
	let mut number = 0;
	let mut numpos = 0;
	for char in cs.chars() {
		let char = char.to_ascii_lowercase();
		if char == '#' && numpos == 0 { continue; }
		if ('a'..='f').contains(&char) || ('0'..='9').contains(&char) && numpos < 6 {
			let value:u32 = match char {
				'0'=>0, '1'=>1, '2'=>2, '3'=>3, '4'=>4, '5'=>5,
				'6'=>6, '7'=>7, '8'=>8, '9'=>9, 'a'=>10, 'b'=>11,
				'c'=>12, 'd'=>13, 'e'=>14, 'f'=>15,
				_ => 0
			};
			number += 16u32.pow(5-numpos) * value;
			
			numpos += 1;
		} else { return Err(()); }
	}
	
	Ok(match co {
		ChannelOrder::Rgba => (number << 8)+255,
		ChannelOrder::Argb => number+4278190080,
		ChannelOrder::Rgb  => number & 16777215
	})
}