// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
"use strict";
import * as wasm from './wasm.js';

const screen = document.querySelector('#screen');
const screenCtx = screen.getContext('2d');

// Load wasm module with callback to continue initialization
let loadSuccessCallback = initialize;
wasm.loadModule(loadSuccessCallback);

// Load data and add event listeners
function initialize() {
    // Initialize LCD screen
    screen.height = 536;
    screen.width = 336;
    wasm.init();
    repaint();
}

// Paint the frame buffer (wasm shared memory) to the screen (canvas element)
function repaint() {
    let data = wasm.frameBuf();
    let imageData = screenCtx.getImageData(0, 0, screen.width, screen.height);
    let pxOffset = 0;
    for (let line=0; line<data.lines; line++) {
        for (let w=0; w<data.wordsPerLine; w++) {
            // Lines are padded to multiples of 4 bytes
            if (w*32 < data.pxPerLine) {
                let index = (line * data.wordsPerLine * 4) + w*4;
                let b0 = data.bytes[index];
                let b1 = data.bytes[index+1];
                let b2 = data.bytes[index+2];
                let b3 = data.bytes[index+3];
                let word = ((b3 >>> 0) << 24) | (b2 << 16) | (b1 << 8) | b0;
                for (let bit=0; bit<32; bit++) {
                    let pxOffset = (line * data.pxPerLine + w*32 + bit) * 4;
                    let fbPixel = 1 & (word >> bit)
                    // Pixel == 1 means clear (takes color of backlit background)
                    // Pixel == 0 means black
                    // To let the white (clear) pixels take the color of the canvas element's
                    // background, modulate the alpha channel.
                    imageData.data[pxOffset] = 0x33;
                    imageData.data[pxOffset+1] = 0x33;
                    imageData.data[pxOffset+2] = 0x33;
                    imageData.data[pxOffset+3] = (fbPixel==1) ? 0 : 0xff;
                }
            }
        }
    }
    screenCtx.putImageData(imageData, 0, 0);
}
