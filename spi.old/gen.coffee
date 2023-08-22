#!/usr/bin/env coffee

> fs > readdirSync copyFileSync
  @w5/uridir
  @w5/write
  path > join
  ./esbuild

ROOT = uridir(import.meta)

do =>
  li = readdirSync join ROOT,'src/url'
  jsli = []
  for i from li
    if i.endsWith '.coffee'
      jsli.push i.slice(0,-7)

  js = []
  for i from jsli
    js.push "import #{i} from './url/#{i}.js'"

  js.push "export default {#{jsli.join(',')}}"

  write(
    join ROOT,'lib/map.js'
    js.join('\n')
  )
  await esbuild()
  write(
    join ROOT,'bundle/package.json'
    '{"type": "module"}'
  )
  return

