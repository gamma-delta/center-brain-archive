import { AllDSPInfo, Recipe } from "./dsp";
import * as Dom from "./dom";
import { English } from "./translate/english";
import { isOnMobile } from "./checkers";

import halfmoon = require("halfmoon");

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
        displayUsageLinks: isOnMobile(),
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
            const matcher = /#\?(\w+)=(\w+)/;
            let match = window.location.hash.match(matcher);

            let newContent;

            if (match !== null && match.length == 3) {
                let key = match[1];
                let value = match[2];
                if (key == "production") {
                    newContent = Dom.makeProduceItem(value);
                } else if (key == "consumption") {
                    newContent = Dom.makeConsumeItem(value);
                } else {
                    newContent = Dom.makeLanding();
                }
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