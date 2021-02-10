export type Params = {
    action: "production",
    item: string,
} | {
    action: "consumption",
    item: string,
} | {
    action: "showPins",
} | {
    action: "search",
    search: string,
};

/**
 * Get the params from the URL.
 * Returns null if there was a problem.
 */
export function getParams(): Params | null {
    let raw = new URLSearchParams(window.location.hash.slice(1));
    let params: { [k: string]: string; } = {};
    raw.forEach((val, key) => {
        params[key as string] = val;
    });

    if (params.action === null) {
        return null;
    }

    switch (params.action) {
        case "production": {
            if (params.item) {
                return {
                    action: "production",
                    item: params.item
                };
            } else return null;
        }
        case "consumption": {
            if (params.item) {
                return {
                    action: "consumption",
                    item: params.item
                };
            } else return null;
        }
        case "showPins": return { action: "showPins" };
        case "search": {
            if (params.search) {
                return {
                    action: "search",
                    search: params.search
                };
            } else return null;
        }
        default: return null;
    }
}
