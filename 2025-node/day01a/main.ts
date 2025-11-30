import { readlines } from '../lib/util';

const lines = readlines(__dirname + '/input');
console.log(JSON.stringify(lines, null, 2));
