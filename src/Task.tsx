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
                next: {props.next_run? (new Date(props.next_run!)).toLocaleString() : "-"}
            </div>
            <div class={styles.actions}>
                <button class="run" onClick={() => wasm.run_task(props.name)}>Run</button>
                <button class="unset" onClick={() => wasm.unset_task(props.name)}>Unset</button>
            </div>
        </div>
    );
}
