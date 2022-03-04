import {MandelbrotCanvas} from "wasmbrot";
import { memory } from "wasmbrot/wasmbrot_bg";

const width = 1000;
const height = 1000;

let mandelbrot = MandelbrotCanvas.new(width, height)
mandelbrot.mandelbrot();

const canvas = document.getElementById("mandelbrot-canvas");
const ctx = canvas.getContext('2d');

const escapeTimesPtr = mandelbrot.pixels();
const escapeTimes = new Uint8ClampedArray(memory.buffer, escapeTimesPtr, 4 * width * height);
console.log(escapeTimes);
const image = new ImageData(escapeTimes, width, height);
console.log(image);
ctx.putImageData(image, 0, 0);