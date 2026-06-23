/**
 * Dioxus Mini Program - WXWebAssembly Glue
 * 
 * This module provides the JavaScript glue code for running Dioxus WASM
 * in WeChat Mini Program using WXWebAssembly.
 * 
 * Reference: https://developers.weixin.qq.com/miniprogram/dev/framework/performance/wasm.html
 */

let wasm = null;
let WASM_VECTOR_LEN = 0;
let cachedTextDecoder = null;
let cachedTextEncoder = null;
let cachedUint8ArrayMemory0 = null;

// WASM 文件路径
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
 * Run the Dioxus application
 */
export async function runDioxus() {
    await initWasm();
    
    // Call the WASM run function
    if (wasm.run) {
        wasm.run();
    }
}

/**
 * Get DOM mutations from WASM
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
 * Apply DOM mutations to Mini Program view
 */
export function applyMutations(mutationsJson) {
    try {
        const mutations = JSON.parse(mutationsJson);
        
        // Process mutations and apply to Mini Program DOM
        for (const mutation of mutations) {
            switch (mutation.type) {
                case 'create':
                    // Create element
                    wx.createSelectorQuery()
                        .select('#dioxus-root')
                        .fields({ node: true, size: true })
                        .exec((res) => {
                            // Handle element creation
                        });
                    break;
                case 'setAttribute':
                    // Set attribute
                    break;
                case 'setText':
                    // Set text content
                    break;
                case 'remove':
                    // Remove element
                    break;
            }
        }
    } catch (error) {
        console.error('[Dioxus] Failed to apply mutations:', error);
    }
}

// Worker message handling
self.onmessage = async function(event) {
    const { type, data } = event.data;
    
    try {
        switch (type) {
            case 'init':
                await initWasm();
                self.postMessage({ type: 'result', success: true });
                break;
                
            case 'run':
                await runDioxus();
                self.postMessage({ type: 'result', success: true });
                break;
                
            case 'getMutations':
                const mutations = await getMutations();
                self.postMessage({ type: 'mutations', data: mutations });
                break;
                
            default:
                console.warn('[Dioxus Worker] Unknown message type:', type);
        }
    } catch (error) {
        self.postMessage({ 
            type: 'error', 
            error: error.message 
        });
    }
};

// Notify main thread that worker is ready
self.postMessage({ type: 'worker_ready' });