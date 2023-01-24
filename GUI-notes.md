# Rust GUI Notes 
### ref: 
https://dev.to/davidedelpapa/rust-gui-introduction-a-k-a-the-state-of-rust-gui-libraries-as-of-january-2021-40gl

## druid 

	cargo new gui-druid

	cd gui-druid

	cargo add druid
	
### main.rs sample code

	use druid::widget::{Button, Flex, Label};
	use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, Data};

	#[derive(Clone, Data)]
	struct Counter(i32);

	fn main() -> Result<(), PlatformError> {
		// Window builder. We set title and size
		let main_window = WindowDesc::new(ui_builder)
			.title("Hello, Druid!")
			.window_size((200.0, 100.0));

		// Data to be used in the app (=state)
		let data: Counter = Counter(0);

		// Run the app
		AppLauncher::with_window(main_window)
			.use_simple_logger() // Neat!
			.launch(data)
	}

	fn ui_builder() -> impl Widget<Counter> {
		// The label text will be computed dynamically based on the current locale and count
		let text = LocalizedString::new("hello-counter")
			.with_arg("count", |data: &Counter, _env| (*data).0.into());
		let label = Label::new(text).padding(5.0).center();

		// Two buttons with on_click callback
		let button_plus = Button::new("+1")
			.on_click(|_ctx, data: &mut Counter, _env| (*data).0 += 1)
			.padding(5.0);
		let button_minus = Button::new("-1")
			.on_click(|_ctx, data: &mut Counter, _env| (*data).0 -= 1)
			.padding(5.0);

		// Container for the two buttons
		let flex = Flex::row()
			.with_child(button_plus)
			.with_spacer(1.0)
			.with_child(button_minus);

		// Container for the whole UI
		Flex::column()
			.with_child(label)
			.with_child(flex)
	}

## fltk 2023 fail

	cargo new gui-fltk

	cd gui-fltk

	cargo add fltk

### main.rs sample code 

	use fltk::{app::*, button::*, frame::*, window::*};

	fn main() {
		let app = App::default();
		let mut wind = Window::new(100, 100, 400, 300, "Hello, FLTK!");
		let mut frame = Frame::new(0, 0, 400, 200, "Boring label");
		let mut but = Button::new(160, 210, 80, 40, "Click me!");

		wind.end();
		wind.show();

		// Remember: Callbacks after initializing the interface
		but.set_callback(move || frame.set_label("Hello, World!"));

		app.run().unwrap();
	}

## gtk - 2023 01 fail 

	cargo new gui-gtk

	cd gui-gtk

	cargo add gtk 

	cargo add gio 

#  iced 2023 01 fail

	cargo new gui-iced

	cd gui-iced

	cargo add iced --git "https://github.com/hecrj/iced.git" --features glow

#  sciter 2023 fail
	sudo apt-get install freeglut3-dev

	wget https://sciter.com/sdk/sciter-sdk.zip

	mkdir sciter-sdk
	
	cd sciter-sdk 

	unzip ../sciter-sdk.zip

	cd sciter-sdk/build.linux/desktop

	make config=debug_x64 (need to mark sciter-sqlite.make in Makefile )

	cp ../sciter-sdk/bin.lnx/x64/libsciter-gtk.so .

	cargo new gui-sciter

	cd gui-sciter

	cargo add sciter-rs

# IUI - 2023 01 fail

	cargo new gui-iui

	cd gui-iui
	
	cargo add iui