const encoder = new TextEncoder();

const greetText = encoder.encode("Hello, world!");

await Deno.writeFile("greet.txt", greetText);