import { AllDSPInfo, Recipe } from "./dsp";
import * as Dom from "./dom";
import { English } from "./translate/english";
import { isOnMobile } from "./checkers";

require("halfmoon/css/halfmoon-variables.min.css");
import halfmoon = require("halfmoon");

window.onload = halfmoon.onDOMContentLoaded;

export let INFO: AllDSPInfo;
export const TRANSLATIONS = English;
export const OPTIONS = {
    displayUsageLinks: isOnMobile(),
    savedRecipes: [] as Recipe[],
};

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