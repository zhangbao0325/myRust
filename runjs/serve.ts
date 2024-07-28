import { serveListener } from "https://deno.land/std@0.201.0/http/server.ts";

const listener = Deno.listen({ port: 8080 });

console.log("server listening on http://localhost:8080");

await serveListener(listener, (request) => {
  const body = `Your user-agent is:\n\n${request.headers.get(
    "user-agent",
  ) ?? "Unknown"}`;

  return new Response(body, { status: 200 });
});

// deno run serve.ts