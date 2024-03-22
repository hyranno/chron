import type { Component } from "solid-js";
import styles from "./Task.module.css";
import * as wasm from "wasm_mod"

export const TaskStatus: Component<wasm.JsTaskInfo> = (props) => {
    return (
        <div class={styles.TaskStatus}>
            <div class={styles.name}>{props.name}</div>
            <div class={props.err? styles.Err : styles.Ok}>
                last result: {props.last_result? props.last_result! : props.err? props.err! : "-"}
            </div>
            <div class={styles.next}>
                next: {props.next_run? props.next_run! : "-"}
            </div>
        </div>
    );
}
