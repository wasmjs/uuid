import { uuidv3, uuidv4 } from '@wasmjs/uuid';

// Examples from https://github.com/kelektiv/node-uuid#quickstart---commonjs-recommended

// v3
// https://github.com/kelektiv/node-uuid#version-3
const MY_NAMESPACE = '1b671a64-40d5-491e-99b0-da01ff1f3341';
console.log('uuidv3:', uuidv3('hello world', MY_NAMESPACE)); // ⇨ '042ffd34-d989-321c-ad06-f60826172424'

//v4
console.log('uuidv4:', uuidv4());
