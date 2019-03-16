import { uuidv3, uuidv4 } from '@wasmjs/uuid';

// Examples from https://github.com/kelektiv/node-uuid#quickstart---commonjs-recommended

// v3
const MY_NAMESPACE = '1b671a64-40d5-491e-99b0-da01ff1f3341';
console.log('uuidv3:', uuidv3('Hello, World!', MY_NAMESPACE)); // ⇨ 'e8b5a51d-11c8-3310-a6ab-367563f20686'

//v4
console.log('uuidv4:', uuidv4());
