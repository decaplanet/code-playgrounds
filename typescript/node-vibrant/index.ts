// This file requires the following dependencies:
// `npm i node-vibrant`

import Vibrant from "node-vibrant";
import path from "path";

const imgPath = path.resolve(__dirname, "./img.png");
Vibrant.from(imgPath).getPalette((err, palette) => {
	console.log(palette?.Vibrant?.getHex());
});
