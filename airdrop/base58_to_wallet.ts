import bs58 from 'bs58';
import prompt from 'prompt-sync';


// Initialize prompt-sync
const prompt_input = prompt();
console.log(" +++++++++++++ Welcome to Typescript CLI +++++++++++++++");


while (true) {
    console.log("1. To input public key");
    console.log("2. If you want to input your Unit8array");
    console.log("3. If you want to break");

    const value = prompt_input("Enter the number: ");

    if (Number(value.trim()) == 1) {
        const publicKey = prompt_input("Paste your public key here: ");
        const byteArray = bs58_to_bytes(publicKey.trim());
        console.log(`The Unit8Array of your public is: ${byteArray}`);
        continue;
    } else if (Number(value.trim()) == 2) {
        const jsonPrivateKey = JSON.parse(prompt_input("Paste your Private Key Here: "));
        const byteArray = bytes_to_bs58(jsonPrivateKey);
        console.log(`The Base Encoded Form of the private key is: ${byteArray}`);
        continue;
    } else if (Number(value.trim()) == 3) {
        break;
    } else {
        console.log("Enter a valid number!");
    }
}


function bs58_to_bytes(input: string) {
    return bs58.decode(input);
}


function bytes_to_bs58(input: number[]) {
    return bs58.encode(input);
}