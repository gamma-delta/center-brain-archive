import { Item, ItemStack, Recipe } from "./dsp";
import { INFO, TRANSLATIONS } from "./main";

/**
 * Create the main page.
 */
export function makeLanding(): HTMLElement {
    let landing = document.createElement("div");
    landing.innerHTML = `
        <h1 class="display-1">${TRANSLATIONS.other.title}</h1>
        <p>${TRANSLATIONS.other.instructions}</p>
    `;

    let items = document.createElement("ul");
    for (let item of Object.keys(INFO.consumption_methods)) {
        items.appendChild(makeItem(item as any, "li"));
    }
    landing.append(items);


    return landing;
}

/**
 * Create an element with all the recipes that produce the given item.
 */
export function makeProduceItem(item: string): HTMLElement {
    let body = document.createElement("div");
    if (item in INFO.production_methods) {
        body.innerHTML = `<h2 class="display-2">${TRANSLATIONS.other.productionMethods(item as Item)}</h2>`;

        let methods = INFO.production_methods[item as keyof typeof INFO.production_methods];
        for (let recipe of methods) {
            let elm = makeRecipe(recipe);
            elm.classList.add("mt-4");
            body.append(elm);
        }
    } else {
        body.innerText = TRANSLATIONS.other.unknownItem(item);
    }
    return body;
}

/**
 * Create an element with all the recipes that consume the given item.
 */
export function makeConsumeItem(item: string): HTMLElement {
    let body = document.createElement("div");

    if (item in INFO.consumption_methods) {
        body.innerHTML = `<h2 class="display-2">${TRANSLATIONS.other.consumptionMethods(item as Item)}</h2>`;

        let methods = INFO.consumption_methods[item as keyof typeof INFO.consumption_methods];
        for (let recipe of methods) {
            let elm = makeRecipe(recipe);
            elm.classList.add("mt-4");
            body.append(elm);
        }
    } else {
        body.innerText = TRANSLATIONS.other.unknownItem(item);
    }
    return body;
}

/**
 * Create an element to view the specified recipe.
 */
function makeRecipe(recipe: string): HTMLElement {
    if (recipe in INFO.recipes) {
        let entry = INFO.recipes[recipe as keyof typeof INFO.recipes];

        let card = document.createElement("div");
        card.classList.add("card");

        let body = document.createElement("div");
        body.classList.add("card-body");

        let inputs = document.createElement("ul");
        inputs.classList.add("list-unstyled");
        for (let input of entry.ingredients) {
            inputs.append(makeItemstack(input, "li"));
        }
        body.appendChild(inputs);

        body.innerHTML += `<p class="bi-arrow-down">
            ${TRANSLATIONS.other.craftinfo(entry.time, entry.made_in, entry.handcraftable)}
        </p>`;

        let outputs = document.createElement("ul");
        outputs.classList.add("list-unstyled");
        for (let output of entry.results) {
            outputs.append(makeItemstack(output, "li"));
        }
        body.appendChild(outputs);

        let footer = document.createElement("div");
        footer.classList.add("card-footer");
        footer.innerHTML = `<p>${TRANSLATIONS.other.unlockedBy(entry.unlocked_by)}</p>`;
        footer.title = TRANSLATIONS.other.recipeID(recipe as any);

        card.append(body, footer);
        return card;
    } else {
        // Error :(
        let card = document.createElement("div");
        card.classList.add("card");
        card.innerText = TRANSLATIONS.other.unknownRecipeID(recipe);
        return card;
    }
}

/**
 * Create an element representing an item.
 */
function makeItem(item: Item, type: keyof HTMLElementTagNameMap = "p"): HTMLElement {
    let elm = document.createElement(type);
    elm.classList.add('dsp-item');
    elm.innerText = TRANSLATIONS.items[item];

    addHandlersItem(elm, item);

    return elm;
}

/**
 * Create an element representing an itemstack.
 */
function makeItemstack(stack: ItemStack, type: keyof HTMLElementTagNameMap = "p"): HTMLElement {
    let elm = document.createElement(type);
    elm.classList.add('dsp-itemstack');
    elm.innerText = `${stack.count}x ${TRANSLATIONS.items[stack.item]}`;

    addHandlersItem(elm, stack.item);

    return elm;
}

/**
 * Curry a function to put on the onclick handler of an item.
 */
function addHandlersItem(elm: HTMLElement, item: Item) {
    elm.onclick = ev => {
        window.location.hash = "?production=" + item;
    };
    elm.oncontextmenu = ev => {
        window.location.hash = "?consumption=" + item;
        // Prevent the menu from actually happening
        ev.preventDefault();
    };
}
