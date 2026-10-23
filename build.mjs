import {spawnSync} from 'node:child_process';import {mkdirSync,renameSync} from 'node:fs';
mkdirSync('build',{recursive:true});
for(const [kind,features] of [['strict',[]],['vulnerable',['--features','vulnerable']],['sink',['--features','sink']]]){const r=spawnSync('stellar',['contract','build','--locked','--out-dir','build',...features],{stdio:'inherit'});if(r.status!==0)process.exit(r.status??1);renameSync('build/delegatelab_contracts.wasm','build/'+kind+'.wasm');}
