import * as fw from "fireworks";
import { firework_add_rocket, memory } from "fireworks/fireworks_bg.wasm";
import * as p5 from 'p5';

const firework = fw.Firework.new();

// Get buffers
const colors = new Uint8Array(memory.buffer, firework.colors(), firework.rocket_buffer_size * 3);
const rockets = new Float64Array(memory.buffer, firework.rockets(), firework.rocket_buffer_size * 2);
const states = new Uint8Array(memory.buffer, firework.states(), firework.rocket_buffer_size);

const particles = new Float64Array(memory.buffer, firework.particles(), firework.particle_buffer_size * 2);
const particle_states = new Uint8Array(memory.buffer, firework.particle_states(), firework.particle_buffer_size);

// Just a helper function
function delay(ms: number) {
    return new Promise( resolve => setTimeout(resolve, ms) );
}

async function setup(p: p5) {
    p.createCanvas(p.windowWidth, p.windowHeight);

    firework.add_rocket({ rgb: 0xffff00, x: 400, height: 170 });

    // Add rockets
    //firework.add(Math.random() * 256, Math.random() * 256, Math.random() * 256, Math.random() * p.windowWidth, Math.random() * 100 + 100);
    // setInterval(() => {
    //     firework.add(Math.random() * 256, Math.random() * 256, Math.random() * 256, Math.random() * p.windowWidth, Math.random() * 100 + 100);
    // }, 2500);

    // (async () => {
    //     while (true) {
    //         firework.add_rocket({ rgb: 0xff0000, x: 100, height: 100 });
    //         await delay(1000);
    //         firework.add_rocket({ rgb: 0x00ff00, x: 200, height: 125 });
    //         await delay(1100);
    //         firework.add_rocket({ rgb: 0x0000ff, x: 300, height: 150 });
    //         await delay(1200);
    //         firework.add_rocket({ rgb: 0xffffff, x: 400, height: 175 });
    //         await delay(1300);
    //     }
    // })();
}

function draw(p: p5) {
    p.colorMode(p.RGB);
    p.background(0, 0, 0, 25);

    // Recalculate positions and parameters
    firework.apply_force();

    for (let r = firework.rocket_buffer_size - 1; r >= 0; r--) {
        if (states[r] === fw.RocketState.Inactive) continue;

        p.stroke(colors[r * 3], colors[r * 3 + 1], colors[r * 3 + 2]);

        if (states[r] === fw.RocketState.Rising) {
            p.strokeWeight(4);
            p.point(rockets[r * 2], p.height + rockets[r * 2 + 1]);
        } else {
            p.strokeWeight(2.5);
            for (let pix = firework.get_max_particles_per_rocket() - 1; pix >= 0; pix--) {
                if (particle_states[fw.Firework.get_particle_index(r, pix)] === fw.ParticleState.Inactive) continue;
                p.point(particles[fw.Firework.get_particle_index(r, pix) * 2], p.height + particles[fw.Firework.get_particle_index(r, pix) * 2 + 1]);
            }
        }
    }
}

const p = new p5((p: p5) => {
    p.setup = () => setup(p);
    p.draw = () => draw(p);
    return p;
});
