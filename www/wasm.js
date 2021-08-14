import * as wasm from "hello-wasm-pack";

async function getAsByteArray(file) {
    return new Uint8Array(await readFile(file))
}
function readFile(file) {
    return new Promise((resolve, reject) => {
        // Create file reader
        let reader = new FileReader();

        // Register event listeners
        reader.addEventListener("loadend", e => resolve(e.target.result))
        reader.addEventListener("error", reject)

        // Read file
        reader.readAsArrayBuffer(file)
    })
}

onmessage = function(fileObj) {
    getAsByteArray(fileObj).then(u8 => {
        wasm.greet(u8);
        postMessage('end');
    });
};
