import { parentPort } from "worker_threads";

parentPort?.on("message", (item: number) => {
	parentPort?.postMessage(item + 90);
});
