use show_image::event;

#[show_image::main]
fn main() -> Result<(), String> {
	let args : Vec<_> = std::env::args().collect();
	if args.len() != 2 {
		return Err(format!("usage: {} IMAGE", args[0]));
	}

	let path = std::path::Path::new(&args[1]);
	let name = path.file_stem().and_then(|x| x.to_str()).unwrap_or("image");

	let image = image::open(path)
		.map_err(|e| format!("Failed to read image from {:?}: {}", path, e))?;

	let image_info = show_image::image_info(&image).map_err(|e| e.to_string())?;
	println!("{:#?}", image_info);

	let window = show_image::create_window("image", Default::default())
		.map_err(|e| e.to_string())?;
	window.set_image(name, image)
		.map_err(|e| e.to_string())?;

	window.add_event_handler(|window, event, _control| {
		if let event::WindowEvent::KeyboardInput { input, .. } = event {
			if input.virtual_keycode == Some(event::VirtualKeyCode::Escape) && input.state == event::ElementState::Pressed {
				let _ = window.destroy();
			}
		}
	}).map_err(|e| e.to_string())?;

	// Wait forever until the window is closed or escape is pressed.
	show_image::context().set_exit_with_last_window(true);
	loop {
		std::thread::park();
	}
}
