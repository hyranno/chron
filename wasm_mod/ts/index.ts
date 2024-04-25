import { chromeExtension } from "@crxjs/vite-plugin";

export function open_working_tab(url: string): Promise<chrome.tabs.Tab> {
    return chrome.tabs.create({
        active: false,
        index: 0,
        pinned: true,
        url: url,
    });
}
export function close_tab(tab: chrome.tabs.Tab) {
    return chrome.tabs.remove(tab.id!);
}


export function fetch_string_by_xpath(tab: chrome.tabs.Tab, xpath: string): Promise<string | null | undefined> {
    return chrome.scripting.executeScript({
        target: {tabId: tab.id!, frameIds: [0]},
        func: (xpath: string) => {
            let node = document.evaluate(xpath, document, null, XPathResult.FIRST_ORDERED_NODE_TYPE).singleNodeValue;
            let str = document.evaluate(`string(${xpath})`, document, null, XPathResult.STRING_TYPE).stringValue;
            let value = node ? str : null;
            return value;
        },
        args: [xpath],
    }).then(injectionResults => {
        return injectionResults[0].result;
    });
}

export function add_to_reading_list(tab: chrome.tabs.Tab, title: String): Promise<void> {
    return chrome.readingList.addEntry({
        url: tab.url || tab.pendingUrl,
        hasBeenRead: false,
        title: title,
    });
}


export function store_serialized(key: string, value: string): Promise<void> {
    return chrome.storage.local.set({[key]: value});
}
export function load_serialized(key: string): Promise<string> {
    return chrome.storage.local.get([key])
        .then((val) => {return val[key]}
    );
}
