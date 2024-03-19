import type { Component } from "solid-js";
import styles from "./Task.module.css";

import {Result, Ok, Err} from './util';

export interface Task {
    name: string,
    nextRun?: Date,
    lastResult?: Result<string, string>
    run(): Promise<Task>;
}

export const TaskStatus: Component<Task> = (props) => {
    return (
        <div class={styles.TaskStatus}>
            <div class={styles.name}>{props.name}</div>
            <div class={props.lastResult?.ok()? styles.Ok : styles.Err}>
                last result: {props.lastResult? props.lastResult.value.toString() : "-"}
            </div>
            <div class={styles.next}>
                next: {props.nextRun? props.nextRun.toString() : "-"}
            </div>
        </div>
    );
}


function openWorkingTab(url: URL): Promise<chrome.tabs.Tab> {
    return chrome.tabs.create({
        active: false,
        index: 0,
        pinned: true,
        url: url.toString(),
    })
}
function fetchXpath(tab: chrome.tabs.Tab, xpath: string): Promise<Result<XPathResult, string>> {
    return chrome.scripting.executeScript({
        target: {tabId: tab.id!},
        func: ((xpath: string) => document.evaluate(xpath, document)) as unknown as () => void,     // forcing type match
        args: [xpath],
    }).then(injected => new Result(
        (injected[0].result instanceof XPathResult) ? new Ok(injected[0].result as XPathResult) : new Err("Failed to fetch.")
    ))
}


interface Checker {
    check: (tab: chrome.tabs.Tab) => Promise<Result<boolean, string>>,
}
export class ChangeChecker implements Checker {
    constructor(public xpath: string, public previous?: string) {}
    check = async (tab: chrome.tabs.Tab): Promise<Result<boolean, string>> => {
        let fetched = (await fetchXpath(tab, (this as  ChangeChecker).xpath)).map(v => v.stringValue);
        let result = fetched.map(v => (v !== this.previous));
        if (result.unwrapOr(false)) {
            this.previous = fetched.unwrapOr("");
        }
        return result;
    }
}

interface Planner {
    planNext: (tab: chrome.tabs.Tab) => Date,
}
export class IntervalPlanner implements Planner {
    constructor(public hours: number, public previous?: Date) {}
    planNext = (tab: chrome.tabs.Tab): Date => {
        let interval = this.hours * 60*60*1000;
        let previous = (this.previous? this.previous!.getTime(): Date.now());
        let delay = Date.now() - previous;
        let next = new Date(previous + interval * (1 + Math.floor(delay / interval)));
        this.previous = next;
        return next;
    }
}

export class WatchUpdateTask implements Task {
    constructor(
        public name: string,
        protected url: URL,
        protected checker: Checker,
        protected planner: Planner,
        public nextRun?: Date,
        public lastResult?:Result<string, string>,
    ){}
    async run(): Promise<Task> {
        let tab = await openWorkingTab(this.url);
        let result = await this.checker.check(tab);
        if (result.unwrapOr(false)) {
            chrome.readingList.addEntry({
                url: this.url.toString(),
                hasBeenRead: false,
                title: tab.title!
            });
        }
        let nextRun = (result.ok()) ? this.planner.planNext(tab) : this.nextRun ;
        let lastResult = result.map(b => b ? "Updated." : "No update.");
        chrome.tabs.remove(tab.id!);
        return new WatchUpdateTask(this.name, this.url, this.checker, this.planner, nextRun, lastResult);
    }
}


