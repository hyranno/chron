
export class Result<T, E> {
    constructor(public value: Ok<T> | Err<E>) {}
    ok() {return this.value instanceof Ok}
    unwrapOr(v: T): T {return (this.value instanceof Ok) ? this.value.value : v;}
    flatMap<T2>(f: (v: T)=>Result<T2, E>): Result<T2, E> {
        return (this.value instanceof Ok) ? f(this.value.value) : new Result<T2, E>(this.value);
    }
    map<T2>(f: (v: T)=>T2): Result<T2, E> {
        return this.flatMap((v) => new Result<T2, E>(new Ok(f(v))));
    }
}
abstract class ResultEnum<T> {
    constructor(public value: T) {}
}
export class Ok<T> extends ResultEnum<T> {
    [Symbol.toStringTag] = "Ok"
}
export class Err<T> extends ResultEnum<T> {
    [Symbol.toStringTag] = "Err"
}



function openWorkingTab(url: string): Promise<chrome.tabs.Tab> {
    return chrome.tabs.create({
        active: false,
        index: 0,
        pinned: true,
        url: url,
    })
}
function closeTab(tab: chrome.tabs.Tab) {
    chrome.tabs.remove(tab.id!);
}


function fetchStringByXpath(tab: chrome.tabs.Tab, xpath: string): Promise<string | undefined> {
    return chrome.scripting.executeScript({
        target: {tabId: tab.id!},
        func: ((xpath: string) => document.evaluate(xpath, document)) as unknown as () => void,     // forcing type match
        args: [xpath],
    }).then(injected => {
        if (!(injected[0].result instanceof XPathResult)) {return null;}
        if ((injected[0].result as XPathResult).resultType != XPathResult.STRING_TYPE) {return null;}
        return (injected[0].result as XPathResult).stringValue;
    });
}

function addToReadingList(tab: chrome.tabs.Tab) {
    chrome.readingList.addEntry({
        url: tab.url!,
        hasBeenRead: false,
        title: tab.title!,
    });
}


