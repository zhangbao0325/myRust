import { print } from "./base.js";
async function hello() {
    return new Promise((resolve, reject) => {
        // Deno.core.print("hello mochen in js\n");
        print("hello mochen in js\n");
        resolve("hello mochen in rust");
    })
}

await hello();

// Deno.core.print("hello mochen in js\n");