#[cfg(desktop)]
mod desktop;
mod watch;

fn main() {
	#[cfg(desktop)]
	desktop::main();
	let context = tauri::generate_context!();
	
	tauri::Builder::default()
		.plugin(watch::Watcher::default())
		.setup(|app| {
			let win = app.get_window("main").unwrap();
			win.set_transparent_titlebar(true);
			win.position_traffic_lights(30.0, 30.0);
			Ok(())
		})
		.on_window_event(|e| {
			if let WindowEvent::Resized(..) = e.event() {
				let win = e.window();
				win.position_traffic_lights(30., 30.);
			}
		})
		.build(context)
		.run(tauri::generate_context!())
}
