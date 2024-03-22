import { For, type Component } from 'solid-js';

import styles from './App.module.css';

import { TaskStatus } from './Task';

import * as wasm from 'wasm_mod';

let tasks = wasm.dummy_task_infos();

const App: Component = () => {
  return (
    <div class={styles.App}>
      <For each={tasks}  fallback={<div>Loading...</div>}>
        {(item) => <TaskStatus {...item}></TaskStatus>}
      </For>
    </div>
  );
};

export default App;
