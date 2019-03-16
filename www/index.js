import { uuidv3, uuidv4, uuidv5 } from '@wasmjs/uuid';

// Examples from https://github.com/kelektiv/node-uuid#quickstart---commonjs-recommended
const MY_NAMESPACE = '1b671a64-40d5-491e-99b0-da01ff1f3341';

// v3
// https://github.com/kelektiv/node-uuid#version-3
console.log('uuidv3:', uuidv3('hello world', MY_NAMESPACE)); // ⇨ '042ffd34-d989-321c-ad06-f60826172424'

//v4
// https://github.com/kelektiv/node-uuid#version-4
console.log('uuidv4:', uuidv4());

// v5
// https://github.com/kelektiv/node-uuid#version-5
console.log('uuidv5:', uuidv5('hello world', MY_NAMESPACE)); // ⇨ '9f282611-e0fd-5650-8953-89c8e342da0b'
