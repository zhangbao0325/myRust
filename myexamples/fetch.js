function print(data) {
    if (typeof data == "string") {
        Deno.core.print(data);
    } else {
        Deno.core.print(JSON.stringify(data));
    }
}

let res = await fetch("http://www.baidu.com");
print(res) 

let body = await res.text();
print(body);