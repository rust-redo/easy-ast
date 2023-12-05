import { join } from 'node:path'
import { Parser } from 'easy-ast.import'
import { Bench } from 'tinybench';

const repos = [
  ['axios', 'lib/axios.js', 3],
  ['rxjs', 'src/index.ts', 3],
  ['nextui/packages/components/', './**/src/index.ts', 3],
  ['antd', 'components/index.ts', 3],
]

function statis(name, map, depth) {
  const modules = Object.keys(map)
  console.log(`${name}, parsed files: ${modules.filter(m => map[m].type === 'local').length},  total modules: ${modules.length}, total links: ${modules.reduce((acc, file) => {
    return acc + (map[file].import?.length ?? 0)
  }, 0)}, depth: ${depth}`)
}
async function run() {
  for (const [name, target, depth] of repos) {
    function createParser() {
      return new Parser({
        root: join(process.cwd(), `../repos/${name}`)
      })
    }

    statis(name, createParser().parse(target, {resolve: false}), 1)

    const bench = new Bench({ time: 200  });
    bench
      .add(`${name} without resolve`, () => {
        createParser().parse(target, { resolve: false })
      })

    for (let i = 1; i <= depth; i++) {
      statis(name, createParser().parse(target, {depth: i}), i)

      bench.add(`${name} with depth=${i}`, () => {
        createParser().parse(target, { depth: i })
      })
    }

    bench.todo('unimplemented bench')

    await bench.run();

    console.table(bench.table());
  }
}

run()