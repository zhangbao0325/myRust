import { Router,Context,Application} from "https://deno.land/x/oak@v12.5.0/mod.ts";
import { oakCors } from "https://deno.land/x/cors/mod.ts";

const channeone = new BroadcastChannel("earth");
// Set onmessage event handler.
channeone.onmessage = (event: MessageEvent) => {
  console.log("channeone");
};

const channetow = new BroadcastChannel("earth");
// Set onmessage event handler.
channetow.onmessage = (event: MessageEvent) => {
  console.log("channetow");
};

const channethree = new BroadcastChannel("earth");
// Set onmessage event handler.
channethree.onmessage = (event: MessageEvent) => {
  console.log("channethree");
};

const router = new Router();
const index = (ctx: Context) => {
    channetow.postMessage({name:"aaaaa"});
    ctx.response.body = `<!DOCTYPE html>
    <html>
      <head><title>Hello oak!</title><head>
      <body>
        <h1>Hello world!</h1>
      </body>
    </html>
  `;
};
router.get("/", index);
const app = new Application();
// Logger
app.use(async (ctx, next) => {
    await next();
    const rt = ctx.response.headers.get("X-Response-Time");
    console.log(`${ctx.request.method} ${ctx.request.url} - ${rt}`);
});
app.use(oakCors());
// Timing
app.use(async (ctx, next) => {
    const start = Date.now();
    await next();
    const ms = Date.now() - start;
    ctx.response.headers.set("X-Response-Time", `${ms}ms`);
});
app.use(router.routes());
app.use(router.allowedMethods());

console.log("开始监听");
//这里的端口可以不填 只有在deno运行时底下是生效的 在当前的这个魔改后的版本是无效的
app.listen({ port: 4000 });