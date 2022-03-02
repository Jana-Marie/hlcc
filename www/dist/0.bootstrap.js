(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/hlcc.js":
/*!**********************!*\
  !*** ../pkg/hlcc.js ***!
  \**********************/
/*! exports provided: compute_input */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _hlcc_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./hlcc_bg.wasm */ \"../pkg/hlcc_bg.wasm\");\n/* harmony import */ var _hlcc_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./hlcc_bg.js */ \"../pkg/hlcc_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"compute_input\", function() { return _hlcc_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"compute_input\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/hlcc.js?");

/***/ }),

/***/ "../pkg/hlcc_bg.js":
/*!*************************!*\
  !*** ../pkg/hlcc_bg.js ***!
  \*************************/
/*! exports provided: compute_input */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"compute_input\", function() { return compute_input; });\n/* harmony import */ var _hlcc_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./hlcc_bg.wasm */ \"../pkg/hlcc_bg.wasm\");\n\n\nlet WASM_VECTOR_LEN = 0;\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _hlcc_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_hlcc_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n/**\n* @param {string} x\n* @returns {number}\n*/\nfunction compute_input(x) {\n    var ptr0 = passStringToWasm0(x, _hlcc_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _hlcc_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    var ret = _hlcc_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"compute_input\"](ptr0, len0);\n    return ret;\n}\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/hlcc_bg.js?");

/***/ }),

/***/ "../pkg/hlcc_bg.wasm":
/*!***************************!*\
  !*** ../pkg/hlcc_bg.wasm ***!
  \***************************/
/*! exports provided: memory, compute_input, __wbindgen_malloc, __wbindgen_realloc */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/hlcc_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _pkg_hlcc_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../pkg/hlcc.js */ \"../pkg/hlcc.js\");\n\n\nvar textBox = document.getElementById('hormone_input');\ntextBox.addEventListener('keypress', function(){\n    if(event.which == 13){\n      let res = Object(_pkg_hlcc_js__WEBPACK_IMPORTED_MODULE_0__[\"compute_input\"])(document.getElementById('hormone_input').value);\n      if (res != -1.0){\n        document.getElementById(\"hormone_output\").innerHTML = \"computes to: \" + Math.round((res + Number.EPSILON) * 100) / 100;\n      } else {\n        document.getElementById(\"hormone_output\").innerHTML = \"Input error!\";\n      }\n      //document.getElementById(\"hormone_output\").classList.remove(\"fade-out\");\n      document.getElementById(\"hormone_output\").classList.add(\"fade-in\");\n      console.log(res);\n    }\n    else if(event.which == 8 || event.which == 13){ //if (document.getElementById('hormone_input').value == \"\")  // why does this not work :(\n      document.getElementById(\"hormone_output\").classList.add(\"fade-out\");\n      document.getElementById(\"hormone_output\").classList.remove(\"fade-in\");\n    }\n});\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);