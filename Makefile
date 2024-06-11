sdl2_window:
	cargo build --bin chip8 --release
	-mkdir -p out 
	cp target/release/chip8 out/chip8 

webasm:
	-mkdir -p www/pkg
	wasm-pack build ./webasm --target web 
	cp ./webasm/www/* ./www/
	cp ./webasm/pkg/webasm.js ./webasm/pkg/webasm_bg.wasm ./www/pkg/

clean: 
	-rm -r ./out/ ./www 
