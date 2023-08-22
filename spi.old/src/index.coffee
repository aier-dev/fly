#!/usr/bin/env coffee

> koa
  msgpackr > pack
  ./map.js

HTTP_PORT = 8080
console.log "http://127.0.0.1:#{HTTP_PORT}/"

(new koa()).use(
  (ctx)=>
    li = ctx.originalUrl.slice(1).split('/')
    if li.length
      func = map[li.shift()]
      if func
        body = (
          await func.apply(ctx,li)
        )
        if body == undefined
          body = ''
          ctx.status = 404
        else
          body = pack body
          ctx.type = 'application/msgpack'
        ctx.body = body
        return
    ctx.body = ''
    ctx.status = 404
    return
).listen(HTTP_PORT)

