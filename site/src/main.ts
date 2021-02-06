import { AllDSPInfo } from "./dsp";
import * as Dom from "./dom";

export let INFO: AllDSPInfo;

fetch('/dsp.json')
    .then(r => r.json())
    .then((data: AllDSPInfo) => {
        INFO = data;
        console.log(INFO);

        const content = document.getElementById('content')!;

        // For now
        content.innerHTML = "";
        content.appendChild(Dom.makeLanding());

        window.addEventListener('hashchange', ev => {
            const matcher = /#\?(\w+)=(\w+)/;
            let match = window.location.hash.match(matcher);

            let newContent;

            if (match !== null && match.length == 3) {
                let key = match[1];
                let value = match[2];
                if (key == "production") {
                    newContent = Dom.makeProduceItem(value);
                } else {
                    newContent = Dom.makeLanding();
                }
            } else {
                newContent = Dom.makeLanding();
            }

            content.innerHTML = "";
            content.appendChild(newContent);
        });
    });