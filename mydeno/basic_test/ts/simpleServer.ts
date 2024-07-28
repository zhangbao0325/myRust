import {
    SimpleResponse,
    SimpleRequest,
    SimpleServer,
  } from "https://raw.githubusercontent.com/notranspile-js/deno-simple-server/master/src/mod.ts";
  
  const server = new SimpleServer({
    listen: { // Deno.ListenOptions | Deno.ListenTlsOptions
      port: 8080,
    },
    files: {
      path: "/web",
      rootDirectory: Deno.cwd(),
      dirListingEnabled: true,
    },
    http: {
      path: "/api/",
      handler: async (req: SimpleRequest): Promise<Response | SimpleResponse> => {
        const msg = await req.json();
        return { // SimpleResponse
          json: {
            received: msg
          }
        };
      },
    },
    websocket: {
      path: "/websocket",
      onmessage: async (sock: WebSocket, ev: MessageEvent) => {
        const data = await Deno.lstat(Deno.cwd());
        const msg = JSON.stringify(data, null, 4);
        sock.send(msg);
      },
    },
    logger: {
      info: (msg: string) => console.log(msg),
      error: (msg: string) => console.log(msg),
    },
    rootRedirectLocation: "/web/",
  });