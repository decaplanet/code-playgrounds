import readline from "readline";

const rl = readline.createInterface({
	input: process.stdin,
	output: process.stdout,
});

rl.question("\nWhat is your name : ", (nameInput) => {
	let isLoop = true;
	let agreementInput = "";

	const askAgreement = () => {
		rl.question("\nCan I spell your name?\nEnter 'y' or 'n'. : ", (answer) => {
			agreementInput = answer.trim();

			if (!agreementInput) {
				askAgreement();
			} else {
				isLoop = false;
				if (agreementInput === "y") {
					for (let item of nameInput) {
						console.log(item);
					}
					console.log(`Here is your name spell, ${nameInput.trim()}!`);
				} else {
					console.log("Exiting the program...");
				}
				rl.close();
			}
		});
	};

	askAgreement();
});
