import {
  readFile,
} from 'node:fs/promises'
import { Project, StructureKind, ts } from 'ts-morph'

const content = await readFile('test-file.txt', 'utf8')

const project = new Project({

})

const sourceFile = project.createSourceFile('morph2.ts', content)

const createApiFactoryAndRegisterActors = sourceFile.getSymbol()?.getExports().filter((symbol) => symbol.getName() === 'createApiFactoryAndRegisterActors')[0]

console.log(createApiFactoryAndRegisterActors?.getDeclarations()[0].getLocal('extHostFileSystem')?.getDeclaredType())
