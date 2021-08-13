import * as wasm from "hello-wasm-pack";

async function getAsByteArray(file) {
    return new Uint8Array(await readFile(file))
}
function readFile(file) {
    return new Promise((resolve, reject) => {
        // Create file reader
        let reader = new FileReader()

        // Register event listeners
        reader.addEventListener("loadend", e => resolve(e.target.result))
        reader.addEventListener("error", reject)

        // Read file
        reader.readAsArrayBuffer(file)
    })
}

document.getElementById("upload").onclick = function () {
    UpladFile().then();
}

async function UpladFile() {
    var fileObj = document.getElementById("file").files[0]; // js 获取文件对象
    const u8 = await getAsByteArray(fileObj);
    console.time('demo');
    wasm.greet(u8)
    console.timeEnd('demo');
}
