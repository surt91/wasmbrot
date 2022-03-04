import {MandelbrotCanvas} from "wasmbrot";
import { memory } from "wasmbrot/wasmbrot_bg";


const canvas = document.getElementById("mandelbrot-canvas");
const ctx = canvas.getContext('2d');

const width = canvas.width;
const height = canvas.height;

let mandelbrot = MandelbrotCanvas.new(width, height)
mandelbrot.mandelbrot();
canvas.onwheel = zoom;
draw();

function draw() {
    const escapeTimesPtr = mandelbrot.pixels();
    const escapeTimes = new Uint8ClampedArray(memory.buffer, escapeTimesPtr, 4 * width * height);
    const image = new ImageData(escapeTimes, width, height);
    ctx.putImageData(image, 0, 0);
}

canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const x = Math.min(Math.floor(canvasLeft), width - 1);
    const y = Math.min(Math.floor(canvasTop), height - 1);

    console.log(x, y);

    mandelbrot.center_on_pixel(x, y);
    mandelbrot.mandelbrot();

    draw();
});

function zoom(event) {
    event.preventDefault();

    mandelbrot.zooming(event.deltaY);
    mandelbrot.mandelbrot();
    draw();
}


