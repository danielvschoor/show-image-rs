use show_image::Event;
use show_image::ImageData;
use show_image::make_window;
use show_image::tch::TensorAsImage;

fn main() -> Result<(), String> {
	let args : Vec<_> = std::env::args().collect();
	if args.len() != 2 {
		return Err(format!("usage: {} IMAGE", args[0]));
	}

	let path = std::path::Path::new(&args[1]);
	let name = path.file_stem().and_then(|x| x.to_str()).unwrap_or("image");

	let tensor = tch::vision::imagenet::load_image(path)
		.map_err(|e| format!("failed to load image from {:?}: {}", path, e))?;
	let tensor = tch::vision::imagenet::unnormalize(&tensor).unwrap();
	let image = tensor.as_image_guess_rgb();
	if let Ok(image) = &image {
		println!("{:#?}", image.info());
	}

	let window = make_window("image")?;
	window.set_image(name, image)?;

	for event in window.events()? {
		if let Event::KeyboardEvent(event) = event {
			//println!("{:#?}", event);
			if event.key == show_image::KeyCode::Escape {
				break;
			}
		}
	}

	show_image::stop()?;
	Ok(())
}
