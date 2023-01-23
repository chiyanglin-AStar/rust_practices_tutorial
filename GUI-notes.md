# Rust GUI Notes 
ref: https://dev.to/davidedelpapa/rust-gui-introduction-a-k-a-the-state-of-rust-gui-libraries-as-of-january-2021-40gl

## druid 

	cargo new GUI-rust-ui-druid

	cd GUI-rust-ui-druid

	cargo add druid

	cargo add pango-sys

	cargo add atk

## fltk 

	cargo new rust-ui-fltk

	cd rust-ui-fltk

	cargo add fltk

## gtkcd ..

	cargo new gui-gtk

	cd gui-gtk

	cargo add gtk --features=v3_16

	cargo add gio --features=v2_44