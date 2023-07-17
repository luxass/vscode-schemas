export type Schema = BuiltinSchema | ExtensionSchema;

export interface BuiltinSchema {
  kind: "builtin"
  name: string
}

export interface ExtensionSchema {
  kind: "extension"
  name: string
}
