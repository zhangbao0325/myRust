Deno.core.print("Resloveing module base.js\n");

function print(s) {
    Deno.core.print(s+ "\n")
}

export {print}