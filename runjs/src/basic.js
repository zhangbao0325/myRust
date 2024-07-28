import { print } from "./base.js";

// Deno.core.print("this is print from basic.js\n");
// print();

async function hello() {
    return new Promise((resolve, reject) => {
        print("this is pring from basic.js");
        resolve("hello world");
    });
}

await hello();