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


async function getTasksJSON() {
  let json_str = await wasm.load_tasks_json();
  let url = URL.createObjectURL(new Blob([json_str]));
  let a = document.createElement('a');
  a.href = url;
  a.download = "chron_tasks.json";
  a.click();
}

async function storeTasksJSON() {
  let file = (document.getElementById("json_file") as HTMLInputElement).files![0];
  let json_str = await file.text();
  wasm.store_tasks_json(json_str)
    .then((_) => alert("saved!"))
    .catch((e) => {alert("something wrong"); console.log(e)})
  ;
}



const Options: Component = () => {
  return (
    <div>
      <button id="download_json" onclick={getTasksJSON}>Get Tasks JSON</button>
      <input id="json_file" onchange={storeTasksJSON} type="file" accept="application/json" name="Store Tasks JSON" />
    </div>
  );
};


render(() => <Options />, root!);
