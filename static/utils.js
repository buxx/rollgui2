export function js_sleep(ms) {
    let id;
    const promise = new Promise(resolve => id = setTimeout(resolve, ms));
    return [promise, id];
}
export function clear_timeout(x) {
    clearTimeout(x)
}