/**
 * Dioxus Mini Program - WASM Wrapper
 * 
 * This module provides a wrapper for loading and using Dioxus WASM
 * in WeChat Mini Program pages.
 * 
 * Usage:
 *   import { initDioxus, runDioxusApp } from '../../utils/wasm.js';
 *   
 *   Page({
 *     async onLoad() {
 *       await initDioxus();
 *       runDioxusApp();
 *     }
 *   });
 */

let wasm = null;
let WASM_VECTOR_LEN = 0;
let cachedTextDecoder = null;
let cachedTextEncoder = null;
let cachedUint8ArrayMemory0 = null;

// WASM 文件路径 - 相对于小程序根目录
const WASM_PATH = '/pkg/dioxus_miniprogram_bg.wasm';

/**
 * Initialize memory caches
 */
function initMemory() {
    cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
    cachedTextDecoder.decode();
    cachedTextEncoder = new TextEncoder();
    if (!('encodeInto' in cachedTextEncoder)) {
        cachedTextEncoder.encodeInto = function (arg, view) {
            const buf = cachedTextEncoder.encode(arg);
            view.set(buf);
            return { read: arg.length, written: buf.length };
        };
    }
}

/**
 * Get Uint8Array memory view
 */
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

/**
 * Decode text from WASM memory
 */
function decodeText(ptr, len) {
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

/**
 * Get string from WASM
 */
function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

/**
 * Pass string to WASM
 */
function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;
    const mem = getUint8ArrayMemory0();
    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);
        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

/**
 * Initialize WASM module using WXWebAssembly
 * 
 * @returns {Promise} Resolves when WASM is loaded
 */
export async function initWasm() {
    if (wasm) {
        return wasm;
    }

    try {
        console.log('[Dioxus] Loading WASM module...');
        
        // Check if WXWebAssembly is available
        if (typeof WXWebAssembly === 'undefined') {
            throw new Error('WXWebAssembly is not available. Please use base library version 2.15.0 or higher.');
        }

        // Use WXWebAssembly.instantiate to load WASM
        const { instance } = await WXWebAssembly.instantiate(WASM_PATH, {
            "./dioxus_miniprogram_bg.js": {
                __wbindgen_init_externref_table: function() {
                    const table = instance.exports.__wbindgen_externrefs;
                    const offset = table.grow(4);
                    table.set(0, undefined);
                    table.set(offset + 0, undefined);
                    table.set(offset + 1, null);
                    table.set(offset + 2, true);
                    table.set(offset + 3, false);
                }
            }
        });
        
        wasm = instance.exports;
        
        // Start WASM module
        if (typeof wasm.__wbindgen_start === 'function') {
            wasm.__wbindgen_start();
        }
        
        initMemory();
        
        console.log('[Dioxus] WASM module loaded successfully');
        return wasm;
    } catch (error) {
        console.error('[Dioxus] Failed to load WASM module:', error);
        throw error;
    }
}

/**
 * Initialize Dioxus application
 * 
 * @returns {Promise} Resolves when Dioxus is initialized
 */
export async function initDioxus() {
    await initWasm();
    console.log('[Dioxus] Application initialized');
}

/**
 * Run the Dioxus application
 * 
 * This starts the main event loop and renders the UI
 */
export function runDioxusApp() {
    if (!wasm) {
        throw new Error('WASM not initialized. Call initDioxus() first.');
    }
    
    // Call the WASM run function
    if (wasm.run) {
        wasm.run();
    }
    
    console.log('[Dioxus] Application running');
}

/**
 * Get DOM mutations from WASM
 * 
 * @returns {string|null} JSON string of mutations
 */
export async function getMutations() {
    await initWasm();
    
    if (wasm.get_mutations) {
        const ptr = wasm.get_mutations();
        const len = wasm.get_mutations_len();
        return getStringFromWasm0(ptr, len);
    }
    return null;
}

/**
 * Get the WASM instance
 * 
 * @returns {object} WASM exports
 */
export function getWasm() {
    return wasm;
}

/**
 * Check if WASM is initialized
 * 
 * @returns {boolean}
 */
export function isWasmReady() {
    return wasm !== null;
}