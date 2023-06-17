#!/usr/bin/env coffee

> @w5/pg/PG > LI ONE0

< (id)->
  if id
    id = Number.parseInt(id)
    if not id
      return
    r = ONE0"SELECT name FROM img.sampler WHERE id=#{id}"
  else
    r = await LI"SELECT id,name FROM img.sampler"
    r = r.flat()
  r
