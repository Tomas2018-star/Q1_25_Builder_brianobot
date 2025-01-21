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
        const image = await readFile("/Users/Apple/Documents/turbin3/solana-starter/ts/cluster1/batman.jpg");
        console.log("🌠 Image File Size: ", image.length);

        //2. Convert image to generic file.
        const file = createGenericFile(
            image, 
            "brian_obot.png", 
            {contentType: "image/jpg"}
        );
        console.log("📂 file = ", file);
        
        //3. Upload image to a decentralized storage,
        // the argument to the upload function is an array of files. and in our case we have just one file.
        // return value is an array of URIs, and in our case we have just one URI.
        const [imageUri] = await umi.uploader.upload([file]); 
        // the url returned might contained a deprecated base url, so we need to replace it with the new one.
        const cleanedUrl = imageUri.replace("https://arweave.net", "https://devnet.irys.xyz");
        console.log("Your image URI: ", cleanedUrl);
    }
    catch(error) {
        console.log("❌ Oops.. Something went wrong", error);
    }
})();
