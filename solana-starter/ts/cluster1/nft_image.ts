import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        const image = await readFile("/Users/Apple/Documents/turbin3/solana-starter/ts/cluster1/generug.png");
        console.log("🌠 Image File = ", image);

        //2. Convert image to generic file.
        const file = createGenericFile(image, "brian_obot.png", {contentType: "image/png"});
        console.log("📂 file = ", file);
        
        //3. Upload image
        const [myUri] = await umi.uploader.upload([file]); 
        console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
