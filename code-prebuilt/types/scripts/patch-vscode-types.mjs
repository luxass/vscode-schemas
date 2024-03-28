import process from 'node:process'
// import { builders, createNode, generateCode, parseModule } from 'magicast'

export async function run() {
  // eslint-disable-next-line no-console
  console.log('Hello, world!')
}

run().catch((err) => {
  console.error(err)
  process.exit(1)
})
