
import { ChangeChecker, IntervalPlanner, Task, WatchUpdateTask } from './Task';

export function dummy_tasks(): Task[] {
    let tasks = [
        new WatchUpdateTask(
            "time0", new URL("https://www.time-j.net/worldtime/country/jp"),
            new ChangeChecker('string(//*[@id="currentTime"])'), new IntervalPlanner(1/60)
        ),
        new WatchUpdateTask(
            "time1", new URL("https://time.is/ja/"),
            new ChangeChecker('string(//*[@id="clock"])'), new IntervalPlanner(1/60)
        ),
    ];
    return tasks;
}
