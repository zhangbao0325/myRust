let file = await Deno.open("./greet.txt", { read: true });
await Deno.copy(file, Deno.stdout);

file.close();