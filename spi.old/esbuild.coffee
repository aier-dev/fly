#!/usr/bin/env coffee

> esbuild > build
  path > join dirname basename
  @w5/uridir
  fs > mkdirSync existsSync

ROOT = uridir(import.meta)

LIB = join ROOT, 'lib'

BUNDLE = join ROOT, 'bundle'

node_modules = join ROOT,'node_modules'

mkdirSync BUNDLE,{recursive:true}

# API = join LIB,'api'
# external = '!/CONF !/CONST/PG_UINT.js @w5/lib ../../../ru/ru/lib/lib.node'.split(' ')

# console.log external
#
bundle = (js)=>
  fp = join LIB, js
  outfile = join BUNDLE, js
  r = await build(
    {
      outfile
      target:"node20"
      platform:"node"
      format: "esm"
      absWorkingDir: ROOT
      bundle: true
      logLevel: "info"
      banner:
        js: '''import {dirname as _dirname_} from 'path';import { createRequire as _createRequire_ } from 'module';const require = _createRequire_(import.meta.url); const __dirname=_dirname_(decodeURI((new URL(import.meta.url)).pathname));'''
      entryPoints: [
        fp
      ]
      plugins: [
        {
          name: 'resolve-js',
          setup: (build) =>
            build.onResolve(
              { filter: /.*/ }
              ({path, kind, resolveDir}) =>
                if kind == 'entry-point'
                  return

                if path.startsWith('@') and path.split('/').length > 2
                  fp = join node_modules, path
                  if not existsSync fp
                    return {
                      path:join node_modules,path+'.js'
                    }

                # console.log {resolveDir}, kind, path
                # if path.startsWith '@w5/'
                # if resolveDir.startsWith(LIB) or not resolveDir.startsWith '/'
                #   if not path.endsWith '.js'
                #     c = path.charAt(0)
                #     if c == '~' and path.charAt(1) == '/'
                #       base = API
                #       path = path[2..]
                #     else if c == '.'
                #       base = resolveDir
                #     if base
                #       path = join base, path+'.js'
                #       return { path }
                return

            )
        }
      ]
    }
  )
#     plugins: [
#     ]
#     outfile
#     #minify: true
#     platform:"node"
#     format: "esm"
#     external
#   }).catch =>
#     process.exit(1)
#   if r.errors.length
#     console.log fp, r
#   console.log outfile
#   return
#
< =>
  Promise.all [
    "index.js"
    # "api/fork.js"
    # "api/boot.js"
    # "Init/main.js"
  ].map bundle

