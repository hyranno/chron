import { chromeExtension } from "@crxjs/vite-plugin";

export function open_working_tab(url: string): Promise<chrome.tabs.Tab> {
    return chrome.tabs.create({
        active: false,
        index: 0,
        pinned: true,
        url: url,
    })
}
export function close_tab(tab: chrome.tabs.Tab) {
    chrome.tabs.remove(tab.id!);
}


export function fetch_string_by_xpath(tab: chrome.tabs.Tab, xpath: string): Promise<string | null | undefined> {
    return chrome.scripting.executeScript({
        target: {tabId: tab.id!},
        func: ((xpath: string) => document.evaluate(xpath, document)) as unknown as () => void,     // forcing type match
        args: [xpath] as unknown as [],
    }).then(injected => {
        if (!(injected[0].result as any instanceof XPathResult)) {return null;}
        if ((injected[0].result as unknown as XPathResult).resultType != XPathResult.STRING_TYPE) {return null;}
        return (injected[0].result as unknown as XPathResult).stringValue;
    });
}

export function add_to_reading_list(tab: chrome.tabs.Tab) {
    chrome.readingList.addEntry({
        url: tab.url!,
        hasBeenRead: false,
        title: tab.title!,
    });
}


export function util_alert() {
    alert("util.alert");
}


export function store_serialized(key: string, value: string): Promise<void> {
    return chrome.storage.sync.set({[key]: value});
}
export function load_serialized(key: string): Promise<string> {
    return chrome.storage.sync.get([key])
        .then((val) => {return val[key]}
    );
}
