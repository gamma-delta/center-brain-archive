import { AllDSPInfo, Recipe } from "./dsp";
import * as Dom from "./dom";
import { English } from "./translate/english";

import halfmoon = require("halfmoon");
import { getParams } from "./params";

export let INFO: AllDSPInfo;
export const TRANSLATIONS = English;

const optionsKey = "CenterBrainArchiveOptions";
export let OPTIONS: {
    displayUsageLinks: boolean,
    savedRecipes: Recipe[];
};
let savedOptions = localStorage.getItem(optionsKey);
if (savedOptions != null) {
    OPTIONS = JSON.parse(savedOptions);
} else {
    OPTIONS = {
        displayUsageLinks: true,
        savedRecipes: [],
    };
};

window.addEventListener("load", halfmoon.onDOMContentLoaded);
window.addEventListener("visibilitychange", ev => {
    localStorage.setItem(optionsKey, JSON.stringify(OPTIONS));
});

fetch('dsp.json')
    .then(r => r.json())
    .then((data: AllDSPInfo) => {
        INFO = data;
        console.log(INFO);

        const content = document.getElementById('content')!;
        let updater = () => {
            let newContent;
            let params = getParams();
            if (params === null) {
                newContent = Dom.makeLanding();
            } else if (params.action == "consumption") {
                newContent = Dom.makeConsumeItem(params.item);
            } else if (params.action == "production") {
                newContent = Dom.makeProduceItem(params.item);
            } else if (params.action == "showPins") {
                newContent = Dom.makeViewPins();
            } else {
                newContent = Dom.makeLanding();
            }

            content.innerHTML = "";
            content.appendChild(newContent);
        };

        window.addEventListener('hashchange', updater);
        // and call it to start up!
        updater();
        Dom.updatePins();

        const usageLinkToggle = document.getElementById('usage-links')! as HTMLInputElement;
        usageLinkToggle.checked = OPTIONS.displayUsageLinks;
        usageLinkToggle.onclick = ev => {
            OPTIONS.displayUsageLinks = usageLinkToggle.checked;
            updater();
            Dom.updatePins();
        };
        const sidebarToggle = document.getElementById('toggle-sidebar-btn')!;
        sidebarToggle.onclick = () => {
            halfmoon.toggleSidebar();
        };

        document.getElementById('sidebar-title')!.innerText = TRANSLATIONS.other.pinnedRecipes;
    });