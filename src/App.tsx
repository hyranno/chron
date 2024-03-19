import { For, type Component } from 'solid-js';

import styles from './App.module.css';

import {dummy_tasks} from './storage';
import { TaskStatus } from './Task';

import * as wasm from '../wasm_mod/pkg/wasm_mod';

let tasks = dummy_tasks();

const App: Component = () => {
  wasm.hello_content();
  return (
    <div class={styles.App}>
      <For each={tasks}  fallback={<div>Loading...</div>}>
        {(item) => <TaskStatus {...item}></TaskStatus>}
      </For>
    </div>
  );
};

export default App;
