/* @refresh reload */
import { render } from 'solid-js/web';
import { type Component, createSignal } from 'solid-js';

import * as wasm from 'wasm_mod';


const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}


function store_tasks() {
  let json_str = (document.getElementById("json_input")! as HTMLTextAreaElement).value;
  wasm.store_tasks_json(json_str)
    .then((_) => alert("saved!"))
    .catch((e) => {alert("something wrong"); console.log(e)})
  ;
}


let [tasks, setTasks] = createSignal<string>();
wasm.load_tasks_json().then((v) => setTasks(v));

const Options: Component = () => {
  return (
    <div>
      <textarea id="loaded_tasks" readonly>{tasks()}</textarea>
      <textarea id="json_input"></textarea>
      <button id="store_tasks" onclick={store_tasks}>save</button>
    </div>
  );
};


render(() => <Options />, root!);
