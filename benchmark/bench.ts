import b from 'benny'

import { getData } from '../index'

async function getDataJs(url: string) {
  const response = await fetch(url)

  return response
}

const url = 'https://catfact.ninja/fact'

async function run() {
  await b.suite(
    'Get Data',

    b.add('Native getData', async () => {
      await getData(url)
    }),

    b.add('JavaScript getData', async () => {
      await getDataJs(url)
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
