import { readInput } from '../common.ts'

const input = readInput(import.meta.url)
const res = input.split('').reduce((accum, c) => accum + (c == '(' ? 1 : -1), 0)
console.log(res)
