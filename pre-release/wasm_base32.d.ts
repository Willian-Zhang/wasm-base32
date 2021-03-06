/* tslint:disable */
/* eslint-disable */
/**
* @param {string} encoded 
* @returns {string} 
*/
export function decode_to_string(encoded: string): string | undefined;

/**
* If `module_or_path` is {RequestInfo}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {RequestInfo | BufferSource | WebAssembly.Module} module_or_path
*
* @returns {Promise<any>}
*/
export default function init (module_or_path: RequestInfo | BufferSource | WebAssembly.Module): Promise<any>;
        