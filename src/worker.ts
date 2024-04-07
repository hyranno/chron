import {update_tasks} from 'wasm_mod';

chrome.alarms.onAlarm.addListener((alarm: chrome.alarms.Alarm) => {
    if (alarm.name == "chron") {
        chron();
    }
});
async function chron() {
    await update_tasks();
}

async function checkAlarmState() {
    const alarm = await chrome.alarms.get("chron");
    if (!alarm) {
        await chrome.alarms.create(
            "chron",
            { periodInMinutes: 1 }
        );
    }
}
checkAlarmState();
