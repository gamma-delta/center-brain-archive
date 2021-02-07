/******/ (() => { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ({

/***/ "./src/dom.ts":
/*!********************!*\
  !*** ./src/dom.ts ***!
  \********************/
/***/ ((__unused_webpack_module, exports, __webpack_require__) => {


exports.__esModule = true;
exports.makeConsumeItem = exports.makeProduceItem = exports.makeLanding = void 0;
var main_1 = __webpack_require__(/*! ./main */ "./src/main.ts");
/**
 * Create the main page.
 */
function makeLanding() {
    var landing = document.createElement("div");
    landing.innerHTML = "\n        <h1 class=\"display-1\">Center Brain Archive</h1>\n\n        <p>Left-click on an item to see all the ways to produce it.<br>\n        Right-click to see all the ways to use it.</p>\n    ";
    var items = document.createElement("ul");
    for (var _i = 0, _a = Object.keys(main_1.INFO.consumption_methods); _i < _a.length; _i++) {
        var item = _a[_i];
        items.appendChild(makeItem(item, "li"));
    }
    landing.append(items);
    return landing;
}
exports.makeLanding = makeLanding;
/**
 * Create an element with all the recipes that produce the given item.
 */
function makeProduceItem(item) {
    var body = document.createElement("div");
    if (item in main_1.INFO.production_methods) {
        body.innerHTML = "<h2 class=\"display-2\">Ways to produce " + item + "</h2>";
        var methods = main_1.INFO.production_methods[item];
        for (var _i = 0, methods_1 = methods; _i < methods_1.length; _i++) {
            var recipe = methods_1[_i];
            body.append(makeRecipe(recipe));
        }
    }
    else {
        body.innerText = "Unknown item <code>" + item + "</code>";
    }
    return body;
}
exports.makeProduceItem = makeProduceItem;
/**
 * Create an element with all the recipes that consume the given item.
 */
function makeConsumeItem(item) {
    var body = document.createElement("div");
    if (item in main_1.INFO.consumption_methods) {
        body.innerHTML = "<h2 class=\"display-2\">Ways to consume " + item + "</h2>";
        var methods = main_1.INFO.consumption_methods[item];
        for (var _i = 0, methods_2 = methods; _i < methods_2.length; _i++) {
            var recipe = methods_2[_i];
            body.append(makeRecipe(recipe));
        }
    }
    else {
        body.innerText = "Unknown item <code>" + item + "</code>";
    }
    return body;
}
exports.makeConsumeItem = makeConsumeItem;
/**
 * Create an element to view the specified recipe.
 */
function makeRecipe(recipe) {
    if (recipe in main_1.INFO.recipes) {
        var entry = main_1.INFO.recipes[recipe];
        var card = document.createElement("div");
        card.classList.add("card");
        var header = document.createElement("div");
        header.classList.add("card-header");
        // TODO: translation
        header.innerText = recipe;
        var body = document.createElement("div");
        body.classList.add("card-body");
        var inputs = document.createElement("ul");
        inputs.classList.add("list-unstyled");
        for (var _i = 0, _a = entry.ingredients; _i < _a.length; _i++) {
            var input = _a[_i];
            inputs.append(makeItemstack(input, "li"));
        }
        body.appendChild(inputs);
        body.innerHTML += "<p class=\"bi-arrow-down\">\n            " + entry.time.toPrecision(1) + " seconds, \n            " + entry.made_in + ", \n            " + (entry.handcraftable ? "Handcraftable" : "Not Handcraftable") + "\n        </p>";
        var outputs = document.createElement("ul");
        outputs.classList.add("list-unstyled");
        for (var _b = 0, _c = entry.results; _b < _c.length; _b++) {
            var output = _c[_b];
            outputs.append(makeItemstack(output, "li"));
        }
        body.appendChild(outputs);
        var footer = document.createElement("div");
        footer.classList.add("card-footer");
        footer.innerText = "Unlocked by " + entry.unlocked_by;
        card.append(header, body, footer);
        return card;
    }
    else {
        // Error :(
        var card = document.createElement("div");
        card.classList.add("card");
        card.innerText = "Unknown recipe ID <code>" + recipe + "</code>";
        return card;
    }
}
/**
 * Create an element representing an item.
 */
function makeItem(item, type) {
    if (type === void 0) { type = "p"; }
    var elm = document.createElement(type);
    elm.classList.add('dsp-item');
    elm.innerText = item;
    addHandlersItem(elm, item);
    return elm;
}
/**
 * Create an element representing an itemstack.
 */
function makeItemstack(stack, type) {
    if (type === void 0) { type = "p"; }
    var elm = document.createElement(type);
    elm.classList.add('dsp-itemstack');
    elm.innerText = stack.count + "x " + stack.item;
    addHandlersItem(elm, stack.item);
    return elm;
}
/**
 * Curry a function to put on the onclick handler of an item.
 */
function addHandlersItem(elm, item) {
    elm.onclick = function (ev) {
        window.location.hash = "?production=" + item;
    };
    elm.oncontextmenu = function (ev) {
        window.location.hash = "?consumption=" + item;
        ev.preventDefault();
    };
}


/***/ }),

/***/ "./src/main.ts":
/*!*********************!*\
  !*** ./src/main.ts ***!
  \*********************/
/***/ ((__unused_webpack_module, exports, __webpack_require__) => {


exports.__esModule = true;
exports.INFO = void 0;
var Dom = __webpack_require__(/*! ./dom */ "./src/dom.ts");
fetch('/dsp.json')
    .then(function (r) { return r.json(); })
    .then(function (data) {
    exports.INFO = data;
    console.log(exports.INFO);
    var content = document.getElementById('content');
    // For now
    content.innerHTML = "";
    content.appendChild(Dom.makeLanding());
    window.addEventListener('hashchange', function (ev) {
        var matcher = /#\?(\w+)=(\w+)/;
        var match = window.location.hash.match(matcher);
        var newContent;
        if (match !== null && match.length == 3) {
            var key = match[1];
            var value = match[2];
            if (key == "production") {
                newContent = Dom.makeProduceItem(value);
            }
            else {
                newContent = Dom.makeLanding();
            }
        }
        else {
            newContent = Dom.makeLanding();
        }
        content.innerHTML = "";
        content.appendChild(newContent);
    });
});


/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		if(__webpack_module_cache__[moduleId]) {
/******/ 			return __webpack_module_cache__[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			// no module.id needed
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/************************************************************************/
/******/ 	// startup
/******/ 	// Load entry module
/******/ 	// This entry module is referenced by other modules so it can't be inlined
/******/ 	__webpack_require__("./src/main.ts");
/******/ })()
;
//# sourceMappingURL=center_brain_archive.js.map