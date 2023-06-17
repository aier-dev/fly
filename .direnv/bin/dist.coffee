#!/usr/bin/env coffee

> zx/globals:
  fs > existsSync readFileSync writeFileSync copyFileSync
  path > join
  fs > rmSync
  @w5/mdt/make.js

cwd = process.cwd()

fp = (p)=>
  join cwd,p

src = fp 'src'
lib = fp 'lib'

package_json = 'package.json'

if existsSync src
  if existsSync lib
    rmSync lib, {recursive:true}
  await $'./build.sh'
  # await $'./run.sh'

  package_json_fp = fp package_json
  json = JSON.parse readFileSync(
    package_json_fp
    'utf8'
  )

  writeFileSync(
    package_json_fp
    JSON.stringify json,null,2
  )

  writeFileSync(
    join lib, package_json
    JSON.stringify(json).replaceAll('./lib/','./').replaceAll('"lib/','"./')
  )

  await make cwd

  await $'git add -u'
  await $"sh -c \"git commit -m 'dist' || true\""
  await $'git push'
  cd lib
  #await $'npm publish --access=public'
