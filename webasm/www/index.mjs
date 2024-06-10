import init, { EmulatorHandler } from "../pkg/webasm.js";

await init();

const canvas = document.getElementById("canvas");
const emulator = new EmulatorHandler(canvas);

let emulatorInterval;

document.getElementById("rom").addEventListener("change", (e) => {
    const file = e.target.files[0];
    const reader = new FileReader();

    reader.onload = (e) => {
        const fileData = e.target.result;
        const rom_data = new Uint8Array(fileData);
        emulator.reset();
        emulator.load_rom(rom_data);

        if (emulatorInterval) {
            clearInterval(emulatorInterval);
        }
        emulatorInterval = setInterval(() => {
            for (let i = 0; i < 8; i++) 
                emulator.tick();

            emulator.tick_timers();
            emulator.draw_to_canvas();
        }, 16);
    };
    reader.readAsArrayBuffer(file);
});

console.log(emulator);
