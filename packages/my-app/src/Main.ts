import { MyLib } from "../../my-lib";

export module MyApp {

	export function main(): void {
		console.log(`[TRACE] ### START ###`);
		MyLib.test();
		console.log(`[TRACE] --- END ---`);
	}
}

MyApp.main();
