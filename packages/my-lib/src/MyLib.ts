export class MyLib {

	private constructor() {} 

	public static test(): void {
		console.log(`[TRACE] <MyLib.test> $$$ START $$$`);
		console.log(`[TRACE] <MyLib.test> --- END ---`);
	}
}
