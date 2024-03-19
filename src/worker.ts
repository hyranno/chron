
import {Task} from './Task'


type Data = {
    tasks: Task[],
}

async function chron() {
    let now = Date.now();
    let data = await chrome.storage.sync.get("chron") as Data;
    let taskPromises = data.tasks.map(async task => {
        if (task.nextRun && now < task.nextRun.getTime()) {
            return task;
        } else {
            return await task.run();
        }
    });
    let tasks = await Promise.all(taskPromises);
    data.tasks = tasks;
    chrome.storage.sync.set({"chron": data});
}

