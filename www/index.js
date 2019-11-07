import * as wasm from 'wasm-fractals';

const canvas = document.getElementById('drawing');
const ctx = canvas.getContext('2d');
const realInput = document.getElementById('real');
const imaginaryInput = document.getElementById('imaginary');
const iterationsInput = document.getElementById('iterations');
const renderBtn = document.getElementById('render');
const renderMandlebrotBtn = document.getElementById('render-mandlebrot');
const presets = document.getElementById('selectPreset');

const juliaSetPreSets = [
    { name: '0 + 0i', real: 0, imaginary: 0 },
    { name: '-0.15 + 0.65i', real: -0.15, imaginary: 0.65 },
    { name: '-0.4 + 0.56i', real: -0.4, imaginary: 0.56 },
    { name: '0.3555534 - 0.3372992i', real: 0.3555534, imaginary: -0.3372992 },
    { name: '0.31 + .025i', real: 0.31, imaginary: 0.25 },
    { name: '0.355 + 0.355i', real: 0.355, imaginary: 0.355 },
    { name: '-0.4 + 0.6i', real: -0.4, imaginary: 0.6 },
    { name: '-0.4 - 0.59i', real: -0.4, imaginary: -0.59 },
    { name: '-0.8 + 0.156i', real: -0.8, imaginary: 0.156 },
    { name: '0.274 - 0.008i', real: 0.274, imaginary: -0.008 },
    { name: '-0.123 + 0.745i', real: -0.123, imaginary: 0.745 },
    { name: '-0.75 + 0i', real: -0.75, imaginary: 0 },
    { name: '-0.835 - 0.2321i', real: -0.835, imaginary: -0.2321 },
    { name: '-0.624 + 0.435i', real: -0.624, imaginary: 0.435 },
    { name: '-0.618 + 0i', real: -0.618, imaginary: 0 },
];

let zoom = 1;
let oldX = 0;
let oldY = 0;
let xOffset = 1;
let yOffset = 1;
let isDragging = false;
let mandlebrot = false; // Currently rendering mandlebrot

for (var i = 0; i < juliaSetPreSets.length; i++) {
    var opt = juliaSetPreSets[i];
    var el = document.createElement("option");
    el.textContent = opt.name;
    el.value = opt.name;
    presets.appendChild(el);
}

function getMouseDirection(e) {
    if (isDragging) {
        xOffset += oldX < e.pageX ? 3 : -3;
        yOffset += oldY < e.pageY ? 3 : -3;
        oldX = e.pageX;
        oldY = e.pageY;
        resizeAndDraw()
    }
}

function resizeAndDraw() {
    var w = window.innerWidth;
    var h = window.innerHeight - 130;
    const real = parseFloat(realInput.value) || 0;
    const imaginary = parseFloat(imaginaryInput.value) || 0;
    const iterations = parseInt(iterationsInput.value) || 0;
    canvas.width = w;
    canvas.height = h;
    const size = w > h ? w : h;
    console.log(zoom)
    if (mandlebrot === true) {
        wasm.generate_mandelbrot_image(ctx, size, size, iterations, zoom, xOffset, yOffset);
    } else {
        wasm.generate_julia_image(ctx, size, size, real, imaginary, iterations, zoom, xOffset, yOffset);
    }
}

renderBtn.addEventListener('click', () => {
    mandlebrot = false;
    resizeAndDraw();
});

renderMandlebrotBtn.addEventListener('click', () => {
    mandlebrot = true;
    resizeAndDraw();
});

window.addEventListener('resize', resizeAndDraw);

presets.addEventListener("change", () => {
    mandlebrot = false;
    const selected = juliaSetPreSets.find(x => x.name === presets.value);
    realInput.value = selected.real;
    imaginaryInput.value = selected.imaginary;
    resizeAndDraw();
});

canvas.addEventListener('wheel', function (event) {
    zoom += event.deltaY > 0 ? 0.03 : -0.03;
    resizeAndDraw();
    event.preventDefault();
    return false;
}, false);

canvas.addEventListener("mousemove", getMouseDirection, false);

canvas.addEventListener("mousedown", (e) => {
    isDragging = true;
    oldX = e.pageX;
    oldY = e.pageY;
});

canvas.addEventListener("mouseup", () => {
    isDragging = false;
});

resizeAndDraw();
