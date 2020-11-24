// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
"use strict";
const wasmModule = "demo.wasm";

// Load WASM module, bind shared memory, then invoke callback
export function loadModule(callback) {
    var importObject = {
        js: {js_log_trace: (traceCode) => {
                  console.log("wasm trace code:", traceCode);
              },
            },
    };
    if ("instantiateStreaming" in WebAssembly) {
        // The new, more efficient way
        WebAssembly.instantiateStreaming(fetch(wasmModule), importObject)
            .then(initSharedMemBindings)
            .then(callback);
    } else {
        // Fallback for Safari
        fetch(wasmModule)
            .then(response => response.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, importObject))
            .then(initSharedMemBindings)
            .then(callback);
    }
}

// Bindings for shared memory and functions
var wasmShared;
var wasmExports;
var wasmInstanceReady = false;

// UTF8 decoder
let decoder = new TextDecoder();

// Callback to initialize shared memory IPC bindings once WASM module is instantiated
function initSharedMemBindings(result) {
    wasmExports = result.instance.exports;
    wasmShared = new Uint8Array(wasmExports.memory.buffer);
    wasmInstanceReady = true;
}

export function init() {
    wasmExports.init();
}

export function frameBuf() {
    if (!wasmInstanceReady) {throw "wasm instance is not ready";}
    const lines = 536;
    const wordsPerLine = 11;
    const pxPerLine = 336;
    let size = lines * wordsPerLine * 4;
    let start = wasmExports.frame_buf_ptr();
    let bytes = wasmShared.subarray(start, start + size);
    return {
        bytes: bytes,
        lines: lines,
        wordsPerLine: wordsPerLine,
        pxPerLine: pxPerLine,
    };
}
