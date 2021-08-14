import * as wasm from "hello-wasm-pack";
import JSZip from 'jszip'

console.log(wasm);

async function getAsByteArray(file) {
    return new Uint8Array(await readFile(file))
}

function readFile(file) {
    return new Promise((resolve, reject) => {
        // Create file reader
        let reader = new FileReader()

        // Register event listeners
        reader.addEventListener("loadend", e => resolve(e.target.result))
        reader.addEventListener("error", reject);

        // Read file
        reader.readAsArrayBuffer(file)
    })
}

document.getElementById("rustUpload").onclick = function () {
    RustUploadFile().then();
};

document.getElementById("jsUpload").onclick = function () {
    JSUploadFile().then();
};

async function RustUploadFile() {
    var fileObj = document.getElementById("file").files[0]; // js 获取文件对象
    console.time('rust');
    const u8 = await getAsByteArray(fileObj);
    await new wasm.ZipExtract(u8).extract();
    console.timeEnd('rust');
}


async function JSUploadFile() {
    console.time('js');
    var fileObj = document.getElementById("file").files[0]; // js 获取文件对象
    const zip = await JSZip.loadAsync(fileObj)
    console.timeEnd('js');
}
