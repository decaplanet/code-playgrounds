import path from "path";
import { Worker } from "worker_threads";

const workerPath = path.resolve(__dirname, "./worker");
const numbers = [1, 2, 3, 4, 5, 6, 7];

numbers.forEach((i) => {
	const worker = new Worker(workerPath);

	worker.postMessage(i);
	worker.on("message", (output) => {
		console.log(`Worker for an element ${i} completed.`);
		console.log(`-> ${output}`);
	});
});
